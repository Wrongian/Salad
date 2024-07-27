DROP TABLE IF EXISTS follows;
DROP TABLE IF EXISTS pending_follow_requests;
DROP TABLE IF EXISTS images;
DROP TABLE IF EXISTS links;
DROP TABLE IF EXISTS reset_password_request;
DROP TABLE IF EXISTS users;
DROP FUNCTION IF EXISTS diesel_manage_updated_at(_tbl regclass);
DROP FUNCTION IF EXISTS diesel_set_updated_at();
DROP FUNCTION IF EXISTS reorder_link(node_id INT, new_position_id INT);
DROP TRIGGER IF EXISTS reorder_links_trigger on links;
DROP TRIGGER IF EXISTS reorder_links_on_create_trigger on links;
DROP FUNCTION IF EXISTS reorder_link_after_delete();
DROP FUNCTION IF EXISTS reorder_link_after_create();
