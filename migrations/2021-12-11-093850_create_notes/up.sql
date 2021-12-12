CREATE TABLE notes (
  id SERIAL NOT NULL PRIMARY KEY,
  description TEXT NOT NULL,
  user_id INT NOT NULL,

  CONSTRAINT fk_users
  FOREIGN KEY(user_id)
  REFERENCES users(id)
  ON UPDATE CASCADE
  ON DELETE CASCADE
);