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

-- Seed data for warrior_skills
-- Iterate over each warrior and assign them a random number of random skills
DO $$DECLARE
    warrior_id INT;
    skill_count INT;
    skill_id INT;
BEGIN
    FOR warrior_id IN 1..100 LOOP
        -- Generate a random number of skills between 1 and 20 for each warrior
        skill_count := floor(random() * 20) + 1;
        
        -- Assign random skills to the warrior
        FOR i IN 1..skill_count LOOP
            -- Randomly select a skill_id
            skill_id := floor(random() * 23) + 1;
            
            -- Insert into warrior_skills table
            INSERT INTO warrior_skills (warrior_id, skill_id) VALUES (warrior_id, skill_id);
        END LOOP;
    END LOOP;
END$$;
