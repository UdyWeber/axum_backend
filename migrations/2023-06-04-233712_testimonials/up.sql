-- Your SQL goes here
CREATE TABLE testimonials (
    uuid serial NOT NULL,
    comment character varying(255) NOT NULL,
    commenter character varying(255) NOT NULL,
    project_name character varying(255) NOT NULL,
    CONSTRAINT testimonial_pkey PRIMARY KEY (uuid)
);