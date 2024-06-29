use crate::routes::links_controller::GetImagedLink;
use priority_queue::PriorityQueue;
use std::{cmp::Reverse, collections::HashMap};

// we assume each GetLink element is unique (i.e link.id is unique) in the vector.
pub fn linearise(links: &Vec<GetImagedLink>) -> Vec<GetImagedLink> {
    let mut link_map = HashMap::<i32, GetImagedLink>::new();
    let mut pq = PriorityQueue::<i32, Reverse<i32>>::new();
    let mut linearised = Vec::<GetImagedLink>::new();

    for link in links {
        link_map.insert(link.id, link.clone());
        pq.push(link.id, Reverse(0));
    }

    for link in links {
        if link.next_id.is_some() {
            pq.change_priority_by(&link.next_id.unwrap(), |prev| *prev = Reverse(prev.0 + 1));
        }
    }

    while !pq.is_empty() {
        let curr = pq.pop().unwrap();
        let item = link_map.get(&curr.0).unwrap();
        match item.next_id {
            Some(curr_next_id) => {
                pq.change_priority_by(&curr_next_id, |curr| *curr = Reverse(curr.0 - 1));
            }
            None => {}
        };
        linearised.push(item.clone());
    }

    linearised
}

#[cfg(test)]
mod unit_tests {
    use crate::routes::links_controller::GetImagedLink;

    use super::linearise;

    #[test]
    pub fn it_should_return_linearised() {
        // 3 -> 4 -> 2 -> 1 -> 0
        let unique_links = Vec::<GetImagedLink>::from([
            GetImagedLink {
                id: 0,
                user_id: 0,
                next_id: None,
                description: None,
                title: None,
                href: "".to_string(),
                img_src: None,
            },
            GetImagedLink {
                id: 1,
                user_id: 0,
                next_id: Some(0),
                description: None,
                title: None,
                href: "".to_string(),
                img_src: None,
            },
            GetImagedLink {
                id: 2,
                user_id: 0,
                next_id: Some(1),
                description: None,
                title: None,
                href: "".to_string(),
                img_src: None,
            },
            GetImagedLink {
                id: 3,
                user_id: 0,
                next_id: Some(4),
                description: None,
                title: None,
                href: "".to_string(),
                img_src: None,
            },
            GetImagedLink {
                id: 4,
                user_id: 0,
                next_id: Some(2),
                description: None,
                title: None,
                href: "".to_string(),
                img_src: None,
            },
        ]);
        let result_ids: Vec<i32> = linearise(&unique_links)
            .iter()
            .map(|link| link.id)
            .collect();
        println!("got linearised ids: {:?}", result_ids);
        assert_eq!(result_ids, [3, 4, 2, 1, 0]);
    }
}
