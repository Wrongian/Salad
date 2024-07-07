use aws_sdk_s3::{
    self as s3,
    error::SdkError,
    operation::create_bucket::{CreateBucketError, CreateBucketOutput},
    primitives::ByteStream,
    types::{BucketLocationConstraint, CreateBucketConfiguration},
};
static PROFILE_IMAGE_BUCKET: &str = "profile-images-salad";
static LINK_IMAGE_BUCKET: &str = "link-images-salad";

pub async fn setup_buckets(client: &s3::Client, region: &str) {
    let res = create_bucket(client, region, PROFILE_IMAGE_BUCKET).await;
    let res2 = create_bucket(client, region, LINK_IMAGE_BUCKET).await;
    if !res.is_ok() {
        println!("res failed with: {:?}", res.err());
    }
    if !res2.is_ok() {
        println!("res2 failed with: {:?}", res2.err());
    }
    ()
}

async fn create_bucket(
    client: &s3::Client,
    region: &str,
    bucket_name: &str,
) -> Result<CreateBucketOutput, SdkError<CreateBucketError>> {
    let constraint = BucketLocationConstraint::from(region);
    let cfg = CreateBucketConfiguration::builder()
        .location_constraint(constraint)
        .build();
    client
        .create_bucket()
        .create_bucket_configuration(cfg)
        .bucket(bucket_name)
        .send()
        .await
}

// one profile image per user?
pub async fn get_s3_profile_image(
    client: &s3::Client,
    user_id: String,
) -> Result<ByteStream, String> {
    let get_object = client
        .get_object()
        .bucket(PROFILE_IMAGE_BUCKET)
        .key(format!("{}", user_id))
        .send()
        .await;

    match get_object {
        Ok(res) => Ok(res.body),
        Err(e) => Err(format!("{:?}", e)),
    }
}

// please perform checks before calling this method.
// this method will not check if user_id exists in the postgres db
pub async fn update_s3_profile_image(
    client: &s3::Client,
    profile_img_name: String,
    content: ByteStream,
) -> Result<(), &str> {
    let put_object = client
        .put_object()
        .bucket(PROFILE_IMAGE_BUCKET)
        .key(format!("{}", profile_img_name))
        .body(content)
        .send()
        .await;

    match put_object {
        Ok(res) => Ok(()),
        Err(res) => {
            // TODO: more verbose error message
            Err("failed to update profile image.")
        }
    }
}

pub async fn delete_s3_profile_image(client: &s3::Client, img_name: String) -> Result<(), &str> {
    let delete_response = client
        .delete_object()
        .bucket(PROFILE_IMAGE_BUCKET)
        .key(format!("{}", img_name))
        .send()
        .await;

    match delete_response {
        Ok(res) => Ok(()),
        Err(res) => Err("failed to delete profile image."),
    }
}
pub async fn get_s3_link_image(client: &s3::Client, link_id: String) -> Result<ByteStream, &str> {
    let get_object = client
        .get_object()
        .bucket(LINK_IMAGE_BUCKET)
        .key(format!("{}", link_id))
        .send()
        .await;

    match get_object {
        Ok(res) => Ok(res.body),
        _ => Err("error in getting link image."),
    }
}
// probably not used as a middleware
pub async fn update_s3_link_image(
    client: &s3::Client,
    link_img_name: String,
    content: ByteStream,
) -> Result<(), &str> {
    let put_object = client
        .put_object()
        .bucket(LINK_IMAGE_BUCKET)
        .key(format!("{}", link_img_name))
        .body(content)
        .send()
        .await;

    match put_object {
        Ok(res) => Ok(()),
        Err(res) => {
            // TODO: more verbose error message
            Err("failed to update link image.")
        }
    }
}

pub async fn delete_s3_link_image(client: &s3::Client, img_name: String) -> Result<(), String> {
    let delete_response = client
        .delete_object()
        .bucket(LINK_IMAGE_BUCKET)
        .key(format!("{}", img_name))
        .send()
        .await;

    match delete_response {
        Ok(res) => Ok(()),
        Err(res) => {
            println!("delete image failed with: {:?}", res);
            return Err(String::from("failed to delete image from aws s3"));
        }
    }
}

pub async fn collect_as_bytes(stream: ByteStream) -> Result<Vec<u8>, String> {
    stream
        .collect()
        .await
        .map(|data| data.into_bytes().to_vec())
        .map_err(|_| "failed to stream".to_string())
}

#[cfg(test)]
mod unit_tests {
    use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};
    use aws_sdk_s3::{self as s3, primitives::ByteStream};
    use bytes::Bytes;

    use crate::connectors::buckets::file::{get_s3_profile_image, update_s3_profile_image};

    async fn create_s3_client() -> s3::Client {
        let region_provider = RegionProviderChain::default_provider().or_else("ap-southeast-2");
        let config = aws_config::defaults(BehaviorVersion::latest())
            .region(region_provider)
            .load()
            .await;
        s3::Client::new(&config)
    }
    #[tokio::test]
    async fn it_fails_to_get_non_existent_profile_image() {
        let client = create_s3_client().await;
        let res = get_s3_profile_image(&client, "-123".to_string()).await;
        assert!(!res.is_ok());
    }

    #[tokio::test]
    async fn it_gets_existing_profile_image() {
        let client = create_s3_client().await;
        let res = get_s3_profile_image(&client, "test-profile-image-1.txt".to_string()).await;

        let bytes = res
            .map_err(|e: String| {
                println!("failed to retrieve file: {}", e);
                assert!(false);
            })
            .unwrap()
            .collect()
            .await
            .map(|data| data.into_bytes().to_vec())
            .unwrap();

        let content = String::from_utf8(bytes).unwrap();
        assert_eq!(content, "this is a test profile image.");
    }

    #[tokio::test]
    async fn it_updates_profile_image() {
        let client = create_s3_client().await;
        let res = update_s3_profile_image(
            &client,
            "test-user".to_string(),
            ByteStream::from(Bytes::from_static(b"updated content")),
        )
        .await;
        // update is successful
        assert!(res.is_ok());

        let res = update_s3_profile_image(
            &client,
            "test-user".to_string(),
            ByteStream::from(Bytes::from_static(b"old content")),
        )
        .await;

        // update is successful
        assert!(res.is_ok());
    }
}
