-- Your SQL goes here
CREATE TABLE check_points (
    id integer PRIMARY KEY,
    comments TEXT,
    manager_actions text,
    managee_actions Text,
    highlights Text,
    mood integer,
    goals Text,
    previous_actions_status Text,
    check_point_date TIMESTAMP
);

CREATE TABLE follow_ups (
   id integer  PRIMARY KEY,
   managee_id integer REFERENCES employees(id),
   check_points_ids integer[]
);
