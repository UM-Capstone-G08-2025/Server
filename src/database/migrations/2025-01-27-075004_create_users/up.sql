-- Your SQL goes here
CREATE TABLE "users"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"username" VARCHAR NOT NULL,
	"password" VARCHAR NOT NULL,
	"remember_token" VARCHAR
);

