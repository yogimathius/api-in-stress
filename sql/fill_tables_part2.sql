-- Seed data for warriors
INSERT INTO warriors_2 (id, name, dob) VALUES
('5ef8eb91-6287-47a6-bc29-5a39cc27b110', 'Desmond Doss', '1919-02-07'), -- Desmond Doss, real-life hero and conscientious objector in World War II
('1b304cec-6df6-4722-8638-8dd4280da77d', 'Ghandi', '1869-10-02'), -- Mahatma Gandhi, leader of the Indian independence movement and advocate for nonviolent resistance
('268fbf42-b13d-4d6b-bef9-2b2738b82e8c', 'Aragorn Stormborn II', '2020-11-11'), -- Fictional successor to Aragorn from Lord of the Rings
('92df2baf-0515-4d49-a9a0-1771548ac468', 'Katara Waterbender II', '2025-06-30'), -- Fictional successor to Katara from Avatar: The Last Airbender
('7a961d84-e3d9-4c78-9461-5db1c2dce9dc', 'Wonder Woman II', '2023-03-12'), -- Fictional successor to Wonder Woman from DC Comics
('bda2c18c-5627-4874-84b0-8f1068b5f214', 'Mandela', '1918-07-18'), -- Nelson Mandela, anti-apartheid revolutionary and former President of South Africa
('f69bf3b7-6b6c-49cb-b93d-fb4680d4bf0b', 'Malala', '1997-07-12'), -- Malala Yousafzai, Pakistani activist for female education and the youngest Nobel Prize laureate
('b2dcd9c1-d5ff-4ef1-8360-c6e0df8713ac', 'Sokka Warrior', '2000-04-20'), -- Fictional character Sokka from Avatar: The Last Airbender
('c573a236-9f4e-4f88-85c7-b3aeeb43895b', 'Aslan the Great', '1949-11-29'), -- Fictional lion Aslan from The Chronicles of Narnia
('d895af07-2504-4c7e-b39d-07fcbe58b754', 'Nelson', '1875-06-28'), -- Nelson Mandela, British admiral known for his service during the Napoleonic Wars
('29c46f07-0b40-409f-bc51-0cde4a9d4ab7', 'Rosa Parks', '1913-02-04'), -- Rosa Parks, civil rights activist known for her pivotal role in the Montgomery bus boycott
('d14061d5-1ad7-4c38-a6a5-264b548a2b2d', 'Harriet Tubman', '1822-03-10'), -- Harriet Tubman, abolitionist and political activist known for her work as a "conductor" on the Underground Railroad
('f914937f-3d22-4822-98ec-6d5a9f0f2a4e', 'Eowyn Moonshadow II', '2026-09-15'), -- Fictional successor to Éowyn from Lord of the Rings
('9a22720b-6ee3-4e09-8d3a-ff46b3eb1325', 'Galadriel Lightbringer II', '2024-08-22'), -- Fictional successor to Galadriel from Lord of the Rings
('5dd8e422-8f5f-47d8-9a01-d7e62cf33ac1', 'Beowulf Dragonslayer II', '2028-05-01'), -- Fictional successor to Beowulf from the epic poem
('8e429a14-50ed-4c3d-8581-5d0e27654f3e', 'Joan of Arc II', '2027-01-16'), -- Fictional successor to Joan of Arc from history and legend
('46d5ec24-5973-4cc2-8a88-620cd7a5a61c', 'Robin Hood II', '2029-12-25'), -- Fictional successor to Robin Hood from various folklore and adaptations
('fa63e28b-124e-40f1-af1b-4dcd82ff0653', 'Mulan Swiftblade II', '2030-08-18'), -- Fictional successor to Mulan from Disney's Mulan
('883c91c8-0951-4f8c-88d3-9a421adfa7a0', 'Lara Croft II', '2032-01-31'), -- Fictional successor to Lara Croft from Tomb Raider
('2b8d40e4-6401-45b7-bd8a-e0b3567ffdef', 'Guan Yu II', '2031-10-26'), -- Fictional successor to Guan Yu from Chinese history and folklore
('a61d4a4e-02c1-4710-9dc1-6061d0ef39ac', 'Aloy Braveheart II', '2033-07-07'); -- Fictional successor to Aloy from Horizon Zero Dawn

INSERT INTO skills_2 (name) VALUES
('BJJ'),
('Karate'),
('Judo'),
('KungFu'),
('Capoeira'),
('Boxing'),
('Taekwondo'),
('Aikido'),
('KravMaga'),
('MuayThai'),
('KickBoxing'),
('Pankration'),
('Wrestling'),
('Sambo'),
('Savate'),
('Sumo'),
('Kendo'),
('Hapkido'),
('LutaLivre'),
('WingChu'),
('Ninjutsu'),
('Fencing'),
('ArmWrestling'),
('SuckerPunch'),
('44Magnum'),
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
INSERT INTO warrior_skills_2 (warrior_id, skill_id) VALUES
-- Desmond Doss
('5ef8eb91-6287-47a6-bc29-5a39cc27b110', 15), ('5ef8eb91-6287-47a6-bc29-5a39cc27b110', 18), ('5ef8eb91-6287-47a6-bc29-5a39cc27b110', 20),
-- Ghandi
('1b304cec-6df6-4722-8638-8dd4280da77d', 6), ('1b304cec-6df6-4722-8638-8dd4280da77d', 11), ('1b304cec-6df6-4722-8638-8dd4280da77d', 21),
-- Aragorn Stormborn II
('268fbf42-b13d-4d6b-bef9-2b2738b82e8c', 1), ('268fbf42-b13d-4d6b-bef9-2b2738b82e8c', 3),
-- Katara Waterbender II
('92df2baf-0515-4d49-a9a0-1771548ac468', 2), ('92df2baf-0515-4d49-a9a0-1771548ac468', 16),
-- Wonder Woman II
('7a961d84-e3d9-4c78-9461-5db1c2dce9dc', 4), ('7a961d84-e3d9-4c78-9461-5db1c2dce9dc', 12),
-- Mandela
('bda2c18c-5627-4874-84b0-8f1068b5f214', 7), ('bda2c18c-5627-4874-84b0-8f1068b5f214', 13),
-- Malala
('f69bf3b7-6b6c-49cb-b93d-fb4680d4bf0b', 18), ('f69bf3b7-6b6c-49cb-b93d-fb4680d4bf0b', 20),
-- Sokka Warrior
('b2dcd9c1-d5ff-4ef1-8360-c6e0df8713ac', 9), ('b2dcd9c1-d5ff-4ef1-8360-c6e0df8713ac', 19),
-- Aslan the Great
('c573a236-9f4e-4f88-85c7-b3aeeb43895b', 5), ('c573a236-9f4e-4f88-85c7-b3aeeb43895b', 8),
-- Nelson
('d895af07-2504-4c7e-b39d-07fcbe58b754', 2), ('d895af07-2504-4c7e-b39d-07fcbe58b754', 14),
-- Rosa Parks
('29c46f07-0b40-409f-bc51-0cde4a9d4ab7', 3), ('29c46f07-0b40-409f-bc51-0cde4a9d4ab7', 15),
-- Harriet Tubman
('d14061d5-1ad7-4c38-a6a5-264b548a2b2d', 1), ('d14061d5-1ad7-4c38-a6a5-264b548a2b2d', 17),
-- Eowyn Moonshadow II
('f914937f-3d22-4822-98ec-6d5a9f0f2a4e', 10), ('f914937f-3d22-4822-98ec-6d5a9f0f2a4e', 20),
-- Galadriel Lightbringer II
('9a22720b-6ee3-4e09-8d3a-ff46b3eb1325', 6), ('9a22720b-6ee3-4e09-8d3a-ff46b3eb1325', 12),
-- Beowulf Dragonslayer II
('5dd8e422-8f5f-47d8-9a01-d7e62cf33ac1', 8), ('5dd8e422-8f5f-47d8-9a01-d7e62cf33ac1', 11),
-- Joan of Arc II
('8e429a14-50ed-4c3d-8581-5d0e27654f3e', 4), ('8e429a14-50ed-4c3d-8581-5d0e27654f3e', 16),
-- Robin Hood II
('46d5ec24-5973-4cc2-8a88-620cd7a5a61c', 7), ('46d5ec24-5973-4cc2-8a88-620cd7a5a61c', 13),
-- Mulan Swiftblade II
('fa63e28b-124e-40f1-af1b-4dcd82ff0653', 10), ('fa63e28b-124e-40f1-af1b-4dcd82ff0653', 15),
-- Lara Croft II
('883c91c8-0951-4f8c-88d3-9a421adfa7a0', 1), ('883c91c8-0951-4f8c-88d3-9a421adfa7a0', 16),
-- Guan Yu II
('2b8d40e4-6401-45b7-bd8a-e0b3567ffdef', 3), ('2b8d40e4-6401-45b7-bd8a-e0b3567ffdef', 18),
-- Aloy Braveheart II
('a61d4a4e-02c1-4710-9dc1-6061d0ef39ac', 2), ('a61d4a4e-02c1-4710-9dc1-6061d0ef39ac', 19);
