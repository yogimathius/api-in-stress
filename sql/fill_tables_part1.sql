-- Seed data for warriors
INSERT INTO warriors (name, dob) VALUES
('Aragorn Stormborn', '1975-03-03'), -- Aragorn from Lord of the Rings
('Xena Dragonheart', '1980-09-12'), -- Xena from Xena: Warrior Princess
('Conan Ironclad', '1982-07-06'), -- Conan the Barbarian
('Brienne Starshield', '1986-11-27'), -- Brienne of Tarth from Game of Thrones
('Drizzt Darkblade', '1978-05-12'), -- Drizzt Do'Urden from Forgotten Realms
('Leia Skywatcher', '1977-10-21'), -- Leia Organa from Star Wars
('Geralt Witchslayer', '1979-04-30'), -- Geralt of Rivia from The Witcher
('Buffy Shadowslayer', '1984-01-19'), -- Buffy Summers from Buffy the Vampire Slayer
('Inigo Montoya', '1975-09-10'), -- Inigo Montoya from The Princess Bride
('Alice Nightingale', '1988-02-14'), -- Alice from Resident Evil
('Jon Snow', '1988-12-23'), -- Jon Snow from Game of Thrones
('Ellen Ripley', '1979-06-06'), -- Ellen Ripley from Alien
('Maximus Decimus Meridius', '1970-08-12'), -- Maximus from Gladiator
('Katniss Everdeen', '1993-05-08'), -- Katniss from The Hunger Games
('Legolas Greenleaf', '1987-07-25'), -- Legolas from Lord of the Rings
('Beatrix Kiddo', '1973-04-04'), -- The Bride from Kill Bill
('Luke Skywalker', '1971-09-19'), -- Luke Skywalker from Star Wars
('Indiana Jones', '1969-06-12'), -- Indiana Jones from Indiana Jones series
('Mulan Swiftblade', '1991-02-18'), -- Mulan from Disney's Mulan
('Neo Matrix', '1976-11-07'), -- Neo from The Matrix
('Katara Waterbender', '1990-03-15'), -- Katara from Avatar: The Last Airbender
('Robin Hood', '1980-12-04'), -- Robin Hood from various folklore and adaptations
('Achilles Thunderbolt', '1985-08-28'), -- Achilles from Greek mythology
('Eowyn Moonshadow', '1983-03-01'), -- Éowyn from Lord of the Rings
('Jack Sparrow', '1965-07-23'), -- Jack Sparrow from Pirates of the Caribbean
('William Wallace', '1270-04-10'), -- William Wallace from Braveheart
('Furiosa Warbringer', '1982-10-30'), -- Furiosa from Mad Max: Fury Road
('Han Solo', '1952-05-25'), -- Han Solo from Star Wars
('Gandalf Stormcaller', '1955-11-08'), -- Gandalf from Lord of the Rings
('Hermione Granger', '1979-09-19'), -- Hermione from Harry Potter
('Thorin Oakenshield', '1960-12-05'), -- Thorin from The Hobbit
('Wonder Woman', '1973-07-08'), -- Wonder Woman from DC Comics
('Beowulf Dragonslayer', '500-06-15'), -- Beowulf from the epic poem
('Merida Archer', '1999-03-18'), -- Merida from Disney's Brave
('Captain America', '1918-07-04'), -- Captain America from Marvel Comics
('Samurai Jack', '1250-01-01'), -- Samurai Jack from the animated series
('Sir Lancelot', 'AD 450-11-11'), -- Lancelot from Arthurian legend
('Galadriel Lightbringer', 'TA 1362-09-24'), -- Galadriel from Lord of the Rings
('X-23', '2003-06-21'), -- X-23 from Marvel Comics
('Attila the Hun', 'AD 406-03-17'), -- Attila the Hun from history and legend
('Aloy Braveheart', '3021-02-28'), -- Aloy from Horizon Zero Dawn
('Leonidas Spartan', '540-08-22'), -- Leonidas from 300
('Sir Gawain', 'AD 540-12-20'), -- Sir Gawain from Arthurian legend
('Kenshin Battosai', '1849-06-20'), -- Kenshin from Rurouni Kenshin
('Lara Croft', '1977-02-14'), -- Lara Croft from Tomb Raider
('Guan Yu', 'AD 160-08-14'), -- Guan Yu from Chinese history and folklore
('Hercules', '1250-10-12'), -- Hercules from Greek mythology
('Joan of Arc', '1412-01-06'), -- Joan of Arc from history and legend
('Neo Matrix', '2144-11-02'), -- Neo from The Matrix
('Katniss Everdeen', '1993-05-08'), -- Katniss from The Hunger Games
('Han Solo', '1952-05-25'), -- Han Solo from Star Wars
('Attila the Hun', 'AD 406-03-17'), -- Attila the Hun from history and legend
('Mulan Swiftblade', '1991-02-18'), -- Mulan from Disney's Mulan
('Guan Yu', 'AD 160-08-14'), -- Guan Yu from Chinese history and folklore
('Aloy Braveheart', '3021-02-28'), -- Aloy from Horizon Zero Dawn
('Leonidas Spartan', '540-08-22'), -- Leonidas from 300
('Sir Gawain', 'AD 540-12-20'), -- Sir Gawain from Arthurian legend
('Kenshin Battosai', '1849-06-20'), -- Kenshin from Rurouni Kenshin
('Lara Croft', '1977-02-14'), -- Lara Croft from Tomb Raider
('Hercules', '1250-10-12'), -- Hercules from Greek mythology
('Joan of Arc', '1412-01-06'), -- Joan of Arc from history and legend
('Wonder Woman', '1941-10-21'), -- Wonder Woman from DC Comics
('Captain America', '1918-07-04'), -- Captain America from Marvel Comics
('Samurai Jack', '1250-01-01'), -- Samurai Jack from the animated series
('Sir Lancelot', 'AD 450-11-11'), -- Lancelot from Arthurian legend
('Galadriel Lightbringer', 'TA 1362-09-24'), -- Galadriel from Lord of the Rings
('X-23', '2003-06-21'), -- X-23 from Marvel Comics
('Beowulf Dragonslayer', '500-06-15'), -- Beowulf from the epic poem
('Merida Archer', '1999-03-18'); -- Merida from Disney's Brave


-- Seed data for skills
INSERT INTO skills (name) VALUES
('Swordsmanship'),
('Archery'),
('Magic'),
('Stealth'),
('Leadership'),
('Survival'),
('Alchemy'),
('Tactics'),
('Hand-to-hand Combat'),
('Marksmanship'),
('Sorcery'),
('Diplomacy'),
('Navigation'),
('Intelligence'),
('Tracking'),
('Healing'),
('Engineering'),
('Acrobatics'),
('Animal Handling'),
('Music'),
('Empathy'),
('Negotiation'),
('Persuasion');

-- Seed data for warrior_skills for multiple warriors
INSERT INTO warrior_skills (warrior_id, skill_id) VALUES
-- Aragorn Stormborn
(1, 1), (1, 3), (1, 5), (1, 7), (1, 8), (1, 9), (1, 12), (1, 14), (1, 16), (1, 17),
-- Xena Dragonheart
(2, 2), (2, 4), (2, 6), (2, 10), (2, 11), (2, 13), (2, 15), (2, 18), (2, 19), (2, 20),
-- Conan Ironclad
(3, 1), (3, 3), (3, 5), (3, 7), (3, 8), (3, 9), (3, 12), (3, 14), (3, 16), (3, 17),
-- Brienne Starshield
(4, 2), (4, 4), (4, 6), (4, 10), (4, 11), (4, 13), (4, 15), (4, 18), (4, 19), (4, 20),
-- Drizzt Darkblade
(5, 1), (5, 3), (5, 5), (5, 7), (5, 8), (5, 9), (5, 12), (5, 14), (5, 16), (5, 17),
-- Leia Skywatcher
(6, 2), (6, 4), (6, 6), (6, 10), (6, 11), (6, 13), (6, 15), (6, 18), (6, 19), (6, 20),
-- Geralt Witchslayer
(7, 1), (7, 3), (7, 5), (7, 7), (7, 8), (7, 9), (7, 12), (7, 14), (7, 16), (7, 17),
-- Buffy Shadowslayer
(8, 2), (8, 4), (8, 6), (8, 10), (8, 11), (8, 13), (8, 15), (8, 18), (8, 19), (8, 20),
-- Inigo Montoya
(9, 1), (9, 3), (9, 5), (9, 7), (9, 8), (9, 9), (9, 12), (9, 14), (9, 16), (9, 17),
-- Alice Nightingale
(10, 2), (10, 4), (10, 6), (10, 10), (10, 11), (10, 13), (10, 15), (10, 18), (10, 19), (10, 20),
-- Jon Snow
(11, 1), (11, 3), (11, 5), (11, 7), (11, 8), (11, 9), (11, 12), (11, 14), (11, 16), (11, 17),
-- Ellen Ripley
(12, 2), (12, 4), (12, 6), (12, 10), (12, 11), (12, 13), (12, 15), (12, 18), (12, 19), (12, 20),
-- Maximus Decimus Meridius
(13, 1), (13, 3), (13, 5), (13, 7), (13, 8), (13, 9), (13, 12), (13, 14), (13, 16), (13, 17),
-- Katniss Everdeen
(14, 2), (14, 4), (14, 6), (14, 10), (14, 11), (14, 13), (14, 15), (14, 18), (14, 19), (14, 20),
-- Legolas Greenleaf
(15, 1), (15, 3), (15, 5), (15, 7), (15, 8), (15, 9), (15, 12), (15, 14), (15, 16), (15, 17),
-- Beatrix Kiddo
(16, 2), (16, 4), (16, 6), (16, 10), (16, 11), (16, 13), (16, 15), (16, 18), (16, 19), (16, 20),
-- Luke Skywalker
(17, 1), (17, 3), (17, 5), (17, 7), (17, 8), (17, 9), (17, 12), (17, 14), (17, 16), (17, 17),
-- Indiana Jones
(18, 2), (18, 4), (18, 6), (18, 10), (18, 11), (18, 13), (18, 15), (18, 18), (18, 19), (18, 20),
-- Mulan Swiftblade
(19, 1), (19, 3), (19, 5), (19, 7), (19, 8), (19, 9), (19, 12), (19, 14), (19, 16), (19, 17);
