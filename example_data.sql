PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS entry(
             id INTEGER PRIMARY KEY,
             slug TEXT NOT NULL UNIQUE,
             content TEXT NOT NULL,
             source TEXT NOT NULL,
             link TEXT
         );
INSERT INTO entry VALUES(0,'asdfjkl','Once there was a way to get back home','The Beatles','http://www.metrolyrics.com/once-there-was-a-way-lyrics-beatles.html');
INSERT INTO entry VALUES(1,'jklasdf','I''m a big baby who can punch like a man','Finn, Adventure Time',NULL);
COMMIT;
