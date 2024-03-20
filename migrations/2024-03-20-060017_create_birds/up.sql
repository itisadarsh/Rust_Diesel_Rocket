-- Your SQL goes here
CREATE TABLE bird (
  id SERIAL PRIMARY KEY,
  birdname VARCHAR NOT NULL,
  scientific_name VARCHAR NOT NULL,
  commonwealth_status VARCHAR NOT NULL
);
INSERT INTO bird (birdname, scientific_name, commonwealth_status) VALUES
('Black-eared Miner', 'Manorina melanotis', 'Critically Endangered'),
('Eastern Bristlebir', 'Dasyornis brachypterus', 'Endangered'),
('Tristan Albatross', 'Diomedea exulans exulans', 'Endangered');
