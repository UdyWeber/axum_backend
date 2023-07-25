-- Your SQL goes here
CREATE TABLE IF NOT EXISTS testimonials (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    comment character varying(255) NOT NULL,
    commenter character varying(255) NOT NULL,
    project_name character varying(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS reactions (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    reaction_asset character varying(255) NOT NULL,
    reacter_unique_id character varying(255) NOT NULL,
    project_name character varying(255) NOT NULL
);
