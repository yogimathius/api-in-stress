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
('44Magnum');

-- Seed data for warrior_skills
INSERT INTO warrior_skills_2 (warrior_id, skill_id) VALUES
('5ef8eb91-6287-47a6-bc29-5a39cc27b110', 1), -- Desmond Doss, real-life hero and conscientious objector in World War II
('1b304cec-6df6-4722-8638-8dd4280da77d', 1), -- Mahatma Gandhi, leader of the Indian independence movement and advocate for nonviolent resistance
('268fbf42-b13d-4d6b-bef9-2b2738b82e8c', 1), -- Fictional successor to Aragorn from Lord of the Rings
('92df2baf-0515-4d49-a9a0-1771548ac468', 1), -- Fictional successor to Katara from Avatar: The Last Airbender
('7a961d84-e3d9-4c78-9461-5db1c2dce9dc', 1), -- Fictional successor to Wonder Woman from DC Comics
('bda2c18c-5627-4874-84b0-8f1068b5f214', 1), -- Nelson Mandela, anti-apartheid revolutionary and former President of South Africa
('f69bf3b7-6b6c-49cb-b93d-fb4680d4bf0b', 1), -- Malala Yousafzai, Pakistani activist for female education and the youngest Nobel Prize laureate
('b2dcd9c1-d5ff-4ef1-8360-c6e0df8713ac', 1), -- Fictional character Sokka from Avatar: The Last Airbender
('c573a236-9f4e-4f88-85c7-b3aeeb43895b', 1), -- Fictional lion Aslan from The Chronicles of Narnia
('d895af07-2504-4c7e-b39d-07fcbe58b754', 1), -- Nelson Mandela, British admiral known for his service during the Napoleonic Wars
('29c46f07-0b40-409f-bc51-0cde4a9d4ab7', 1), -- Rosa Parks, civil rights activist known for her pivotal role in the Montgomery bus boycott
('d14061d5-1ad7-4c38-a6a5-264b548a2b2d', 1), -- Harriet Tubman, abolitionist and political activist known for her work as a "conductor" on the Underground Railroad
('f914937f-3d22-4822-98ec-6d5a9f0f2a4e', 1), -- Fictional successor to Éowyn from Lord of the Rings
('9a22720b-6ee3-4e09-8d3a-ff46b3eb1325', 1), -- Fictional successor to Galadriel from Lord of the Rings
('5dd8e422-8f5f-47d8-9a01-d7e62cf33ac1', 1), -- Fictional successor to Beowulf from the epic poem
('8e429a14-50ed-4c3d-8581-5d0e27654f3e', 1), -- Fictional successor to Joan of Arc from history and legend
('46d5ec24-5973-4cc2-8a88-620cd7a5a61c', 1), -- Fictional successor to Robin Hood from various folklore and adaptations
('fa63e28b-124e-40f1-af1b-4dcd82ff0653', 1), -- Fictional successor to Mulan from Disney's Mulan
('883c91c8-0951-4f8c-88d3-9a421adfa7a0', 1), -- Fictional successor to Lara Croft from Tomb Raider
('2b8d40e4-6401-45b7-bd8a-e0b3567ffdef', 1), -- Fictional successor to Guan Yu from Chinese history and folklore
('a61d4a4e-02c1-4710-9dc1-6061d0ef39ac', 1), -- Fictional successor to Aloy from Horizon Zero Dawn
('5ef8eb91-6287-47a6-bc29-5a39cc27b110', 2), -- Desmond Doss, real-life hero and conscientious objector in World War II
('1b304cec-6df6-4722-8638-8dd4280da77d', 2), -- Mahatma Gandhi, leader of the Indian independence movement and advocate for nonviolent resistance
('268fbf42-b13d-4d6b-bef9-2b2738b82e8c', 2), -- Fictional successor to Aragorn from Lord of the Rings
('92df2baf-0515-4d49-a9a0-1771548ac468', 2), -- Fictional successor to Katara from Avatar: The Last Airbender
('7a961d84-e3d9-4c78-9461-5db1c2dce9dc', 2), -- Fictional successor to Wonder Woman from DC Comics
('bda2c18c-5627-4874-84b0-8f1068b5f214', 2), -- Nelson Mandela, anti-apartheid revolutionary and former President of South Africa
('f69bf3b7-6b6c-49cb-b93d-fb4680d4bf0b', 2), -- Malala Yousafzai, Pakistani activist for female education and the youngest Nobel Prize laureate
('b2dcd9c1-d5ff-4ef1-8360-c6e0df8713ac', 2), -- Fictional character Sokka from Avatar: The Last Airbender
('c573a236-9f4e-4f88-85c7-b3aeeb43895b', 2), -- Fictional lion Aslan from The Chronicles of Narnia
('d895af07-2504-4c7e-b39d-07fcbe58b754', 2), -- Nelson Mandela, British admiral known for his service during the Napoleonic Wars
('29c46f07-0b40-409f-bc51-0cde4a9d4ab7', 2), -- Rosa Parks, civil rights activist known for her pivotal role in the Montgomery bus boycott
('d14061d5-1ad7-4c38-a6a5-264b548a2b2d', 2), -- Harriet Tubman, abolitionist and political activist known for her work as a "conductor" on the Underground Railroad
('f914937f-3d22-4822-98ec-6d5a9f0f2a4e', 2), -- Fictional successor to Éowyn from Lord of the Rings
('9a22720b-6ee3-4e09-8d3a-ff46b3eb1325', 2), -- Fictional successor to Galadriel from Lord of the Rings
('5dd8e422-8f5f-47d8-9a01-d7e62cf33ac1', 2), -- Fictional successor to Beowulf from the epic poem
('8e429a14-50ed-4c3d-8581-5d0e27654f3e', 2), -- Fictional successor to Joan of Arc from history and legend
('46d5ec24-5973-4cc2-8a88-620cd7a5a61c', 2), -- Fictional successor to Robin Hood from various folklore and adaptations
('fa63e28b-124e-40f1-af1b-4dcd82ff0653', 2), -- Fictional successor to Mulan from Disney's Mulan
('883c91c8-0951-4f8c-88d3-9a421adfa7a0', 2), -- Fictional successor to Lara Croft from Tomb Raider
('2b8d40e4-6401-45b7-bd8a-e0b3567ffdef', 2), -- Fictional successor to Guan Yu from Chinese history and folklore
('a61d4a4e-02c1-4710-9dc1-6061d0ef39ac', 2), -- Fictional successor to Aloy from Horizon Zero Dawn
('5ef8eb91-6287-47a6-bc29-5a39cc27b110', 3), -- Desmond Doss, real-life hero and conscientious objector in World War II
('1b304cec-6df6-4722-8638-8dd4280da77d', 3), -- Mahatma Gandhi, leader of the Indian independence movement and advocate for nonviolent resistance
('268fbf42-b13d-4d6b-bef9-2b2738b82e8c', 3), -- Fictional successor to Aragorn from Lord of the Rings
('92df2baf-0515-4d49-a9a0-1771548ac468', 3), -- Fictional successor to Katara from Avatar: The Last Airbender
('7a961d84-e3d9-4c78-9461-5db1c2dce9dc', 3), -- Fictional successor to Wonder Woman from DC Comics
('bda2c18c-5627-4874-84b0-8f1068b5f214', 3), -- Nelson Mandela, anti-apartheid revolutionary and former President of South Africa
('f69bf3b7-6b6c-49cb-b93d-fb4680d4bf0b', 3), -- Malala Yousafzai, Pakistani activist for female education and the youngest Nobel Prize laureate
('b2dcd9c1-d5ff-4ef1-8360-c6e0df8713ac', 3), -- Fictional character Sokka from Avatar: The Last Airbender
('c573a236-9f4e-4f88-85c7-b3aeeb43895b', 3), -- Fictional lion Aslan from The Chronicles of Narnia
('d895af07-2504-4c7e-b39d-07fcbe58b754', 3), -- Nelson Mandela, British admiral known for his service during the Napoleonic Wars
('29c46f07-0b40-409f-bc51-0cde4a9d4ab7', 3), -- Rosa Parks, civil rights activist known for her pivotal role in the Montgomery bus boycott
('d14061d5-1ad7-4c38-a6a5-264b548a2b2d', 3), -- Harriet Tubman, abolitionist and political activist known for her work as a "conductor" on the Underground Railroad
('f914937f-3d22-4822-98ec-6d5a9f0f2a4e', 3), -- Fictional successor to Éowyn from Lord of the Rings
('9a22720b-6ee3-4e09-8d3a-ff46b3eb1325', 3), -- Fictional successor to Galadriel from Lord of the Rings
('5dd8e422-8f5f-47d8-9a01-d7e62cf33ac1', 3), -- Fictional successor to Beowulf from the epic poem
('8e429a14-50ed-4c3d-8581-5d0e27654f3e', 3), -- Fictional successor to Joan of Arc from history and legend
('46d5ec24-5973-4cc2-8a88-620cd7a5a61c', 3), -- Fictional successor to Robin Hood from various folklore and adaptations
('fa63e28b-124e-40f1-af1b-4dcd82ff0653', 3), -- Fictional successor to Mulan from Disney's Mulan
('883c91c8-0951-4f8c-88d3-9a421adfa7a0', 3), -- Fictional successor to Lara Croft from Tomb Raider
('2b8d40e4-6401-45b7-bd8a-e0b3567ffdef', 3), -- Fictional successor to Guan Yu from Chinese history and folklore
('a61d4a4e-02c1-4710-9dc1-6061d0ef39ac', 3), -- Fictional successor to Aloy from Horizon Zero Dawn
('5ef8eb91-6287-47a6-bc29-5a39cc27b110', 4), -- Desmond Doss, real-life hero and conscientious objector in World War II
('1b304cec-6df6-4722-8638-8dd4280da77d', 4), -- Mahatma Gandhi, leader of the Indian independence movement and advocate for nonviolent resistance
('268fbf42-b13d-4d6b-bef9-2b2738b82e8c', 4), -- Fictional successor to Aragorn from Lord of the Rings
('92df2baf-0515-4d49-a9a0-1771548ac468', 4), -- Fictional successor to Katara from Avatar: The Last Airbender
('7a961d84-e3d9-4c78-9461-5db1c2dce9dc', 4), -- Fictional successor to Wonder Woman from DC Comics
('bda2c18c-5627-4874-84b0-8f1068b5f214', 4), -- Nelson Mandela, anti-apartheid revolutionary and former President of South Africa
('f69bf3b7-6b6c-49cb-b93d-fb4680d4bf0b', 4), -- Malala Yousafzai, Pakistani activist for female education and the youngest Nobel Prize laureate
('b2dcd9c1-d5ff-4ef1-8360-c6e0df8713ac', 4), -- Fictional character Sokka from Avatar: The Last Airbender
('c573a236-9f4e-4f88-85c7-b3aeeb43895b', 4), -- Fictional lion Aslan from The Chronicles of Narnia
('d895af07-2504-4c7e-b39d-07fcbe58b754', 4), -- Nelson Mandela, British admiral known for his service during the Napoleonic Wars
('29c46f07-0b40-409f-bc51-0cde4a9d4ab7', 4), -- Rosa Parks, civil rights activist known for her pivotal role in the Montgomery bus boycott
('d14061d5-1ad7-4c38-a6a5-264b548a2b2d', 4), -- Harriet Tubman, abolitionist and political activist known for her work as a "conductor" on the Underground Railroad
('f914937f-3d22-4822-98ec-6d5a9f0f2a4e', 4), -- Fictional successor to Éowyn from Lord of the Rings
('9a22720b-6ee3-4e09-8d3a-ff46b3eb1325', 4), -- Fictional successor to Galadriel from Lord of the Rings
('5dd8e422-8f5f-47d8-9a01-d7e62cf33ac1', 4), -- Fictional successor to Beowulf from the epic poem
('8e429a14-50ed-4c3d-8581-5d0e27654f3e', 4), -- Fictional successor to Joan of Arc from history and legend
('46d5ec24-5973-4cc2-8a88-620cd7a5a61c', 4), -- Fictional successor to Robin Hood from various folklore and adaptations
('fa63e28b-124e-40f1-af1b-4dcd82ff0653', 4), -- Fictional successor to Mulan from Disney's Mulan
('883c91c8-0951-4f8c-88d3-9a421adfa7a0', 4), -- Fictional successor to Lara Croft from Tomb Raider
('2b8d40e4-6401-45b7-bd8a-e0b3567ffdef', 4), -- Fictional successor to Guan Yu from Chinese history and folklore
('a61d4a4e-02c1-4710-9dc1-6061d0ef39ac', 4), -- Fictional successor to Aloy from Horizon Zero Dawn
('5ef8eb91-6287-47a6-bc29-5a39cc27b110', 5), -- Desmond Doss, real-life hero and conscientious objector in World War II
('1b304cec-6df6-4722-8638-8dd4280da77d', 5), -- Mahatma Gandhi, leader of the Indian independence movement and advocate for nonviolent resistance
('268fbf42-b13d-4d6b-bef9-2b2738b82e8c', 5), -- Fictional successor to Aragorn from Lord of the Rings
('92df2baf-0515-4d49-a9a0-1771548ac468', 5), -- Fictional successor to Katara from Avatar: The Last Airbender
('7a961d84-e3d9-4c78-9461-5db1c2dce9dc', 5), -- Fictional successor to Wonder Woman from DC Comics
('bda2c18c-5627-4874-84b0-8f1068b5f214', 5), -- Nelson Mandela, anti-apartheid revolutionary and former President of South Africa
('f69bf3b7-6b6c-49cb-b93d-fb4680d4bf0b', 5), -- Malala Yousafzai, Pakistani activist for female education and the youngest Nobel Prize laureate
('b2dcd9c1-d5ff-4ef1-8360-c6e0df8713ac', 5), -- Fictional character Sokka from Avatar: The Last Airbender
('c573a236-9f4e-4f88-85c7-b3aeeb43895b', 5), -- Fictional lion Aslan from The Chronicles of Narnia
('d895af07-2504-4c7e-b39d-07fcbe58b754', 5), -- Nelson Mandela, British admiral known for his service during the Napoleonic Wars
('29c46f07-0b40-409f-bc51-0cde4a9d4ab7', 5), -- Rosa Parks, civil rights activist known for her pivotal role in the Montgomery bus boycott
('d14061d5-1ad7-4c38-a6a5-264b548a2b2d', 5), -- Harriet Tubman, abolitionist and political activist known for her work as a "conductor" on the Underground Railroad
('f914937f-3d22-4822-98ec-6d5a9f0f2a4e', 5), -- Fictional successor to Éowyn from Lord of the Rings
('9a22720b-6ee3-4e09-8d3a-ff46b3eb1325', 5), -- Fictional successor to Galadriel from Lord of the Rings
('5dd8e422-8f5f-47d8-9a01-d7e62cf33ac1', 5), -- Fictional successor to Beowulf from the epic poem
('8e429a14-50ed-4c3d-8581-5d0e27654f3e', 5), -- Fictional successor to Joan of Arc from history and legend
('46d5ec24-5973-4cc2-8a88-620cd7a5a61c', 5), -- Fictional successor to Robin Hood from various folklore and adaptations
('fa63e28b-124e-40f1-af1b-4dcd82ff0653', 5), -- Fictional successor to Mulan from Disney's Mulan
('883c91c8-0951-4f8c-88d3-9a421adfa7a0', 5), -- Fictional successor to Lara Croft from Tomb Raider
('2b8d40e4-6401-45b7-bd8a-e0b3567ffdef', 5), -- Fictional successor to Guan Yu from Chinese history and folklore
('a61d4a4e-02c1-4710-9dc1-6061d0ef39ac', 5), -- Fictional successor to Aloy from Horizon Zero Dawn
('5ef8eb91-6287-47a6-bc29-5a39cc27b110', 6), -- Desmond Doss, real-life hero and conscientious objector in World War II
('1b304cec-6df6-4722-8638-8dd4280da77d', 6), -- Mahatma Gandhi, leader of the Indian independence movement and advocate for nonviolent resistance
('268fbf42-b13d-4d6b-bef9-2b2738b82e8c', 6), -- Fictional successor to Aragorn from Lord of the Rings
('92df2baf-0515-4d49-a9a0-1771548ac468', 6), -- Fictional successor to Katara from Avatar: The Last Airbender
('7a961d84-e3d9-4c78-9461-5db1c2dce9dc', 6), -- Fictional successor to Wonder Woman from DC Comics
('bda2c18c-5627-4874-84b0-8f1068b5f214', 6), -- Nelson Mandela, anti-apartheid revolutionary and former President of South Africa
('f69bf3b7-6b6c-49cb-b93d-fb4680d4bf0b', 6), -- Malala Yousafzai, Pakistani activist for female education and the youngest Nobel Prize laureate
('b2dcd9c1-d5ff-4ef1-8360-c6e0df8713ac', 6), -- Fictional character Sokka from Avatar: The Last Airbender
('c573a236-9f4e-4f88-85c7-b3aeeb43895b', 6), -- Fictional lion Aslan from The Chronicles of Narnia
('d895af07-2504-4c7e-b39d-07fcbe58b754', 6), -- Nelson Mandela, British admiral known for his service during the Napoleonic Wars
('29c46f07-0b40-409f-bc51-0cde4a9d4ab7', 6), -- Rosa Parks, civil rights activist known for her pivotal role in the Montgomery bus boycott
('d14061d5-1ad7-4c38-a6a5-264b548a2b2d', 6), -- Harriet Tubman, abolitionist and political activist known for her work as a "conductor" on the Underground Railroad
('f914937f-3d22-4822-98ec-6d5a9f0f2a4e', 6), -- Fictional successor to Éowyn from Lord of the Rings
('9a22720b-6ee3-4e09-8d3a-ff46b3eb1325', 6), -- Fictional successor to Galadriel from Lord of the Rings
('5dd8e422-8f5f-47d8-9a01-d7e62cf33ac1', 6), -- Fictional successor to Beowulf from the epic poem
('8e429a14-50ed-4c3d-8581-5d0e27654f3e', 6), -- Fictional successor to Joan of Arc from history and legend
('46d5ec24-5973-4cc2-8a88-620cd7a5a61c', 6), -- Fictional successor to Robin Hood from various folklore and adaptations
('fa63e28b-124e-40f1-af1b-4dcd82ff0653', 6), -- Fictional successor to Mulan from Disney's Mulan
('883c91c8-0951-4f8c-88d3-9a421adfa7a0', 6), -- Fictional successor to Lara Croft from Tomb Raider
('2b8d40e4-6401-45b7-bd8a-e0b3567ffdef', 6), -- Fictional successor to Guan Yu from Chinese history and folklore
('a61d4a4e-02c1-4710-9dc1-6061d0ef39ac', 6), -- Fictional successor to Aloy from Horizon Zero Dawn
('5ef8eb91-6287-47a6-bc29-5a39cc27b110', 7), -- Desmond Doss, real-life hero and conscientious objector in World War II
('1b304cec-6df6-4722-8638-8dd4280da77d', 7), -- Mahatma Gandhi, leader of the Indian independence movement and advocate for nonviolent resistance
('268fbf42-b13d-4d6b-bef9-2b2738b82e8c', 7), -- Fictional successor to Aragorn from Lord of the Rings
('92df2baf-0515-4d49-a9a0-1771548ac468', 7), -- Fictional successor to Katara from Avatar: The Last Airbender
('7a961d84-e3d9-4c78-9461-5db1c2dce9dc', 7), -- Fictional successor to Wonder Woman from DC Comics
('bda2c18c-5627-4874-84b0-8f1068b5f214', 7), -- Nelson Mandela, anti-apartheid revolutionary and former President of South Africa
('f69bf3b7-6b6c-49cb-b93d-fb4680d4bf0b', 7), -- Malala Yousafzai, Pakistani activist for female education and the youngest Nobel Prize laureate
('b2dcd9c1-d5ff-4ef1-8360-c6e0df8713ac', 7), -- Fictional character Sokka from Avatar: The Last Airbender
('c573a236-9f4e-4f88-85c7-b3aeeb43895b', 7), -- Fictional lion Aslan from The Chronicles of Narnia
('d895af07-2504-4c7e-b39d-07fcbe58b754', 7), -- Nelson Mandela, British admiral known for his service during the Napoleonic Wars
('29c46f07-0b40-409f-bc51-0cde4a9d4ab7', 7), -- Rosa Parks, civil rights activist known for her pivotal role in the Montgomery bus boycott
('d14061d5-1ad7-4c38-a6a5-264b548a2b2d', 7), -- Harriet Tubman, abolitionist and political activist known for her work as a "conductor" on the Underground Railroad
('f914937f-3d22-4822-98ec-6d5a9f0f2a4e', 7), -- Fictional successor to Éowyn from Lord of the Rings
('9a22720b-6ee3-4e09-8d3a-ff46b3eb1325', 7), -- Fictional successor to Galadriel from Lord of the Rings
('5dd8e422-8f5f-47d8-9a01-d7e62cf33ac1', 7), -- Fictional successor to Beowulf from the epic poem
('8e429a14-50ed-4c3d-8581-5d0e27654f3e', 7), -- Fictional successor to Joan of Arc from history and legend
('46d5ec24-5973-4cc2-8a88-620cd7a5a61c', 7), -- Fictional successor to Robin Hood from various folklore and adaptations
('fa63e28b-124e-40f1-af1b-4dcd82ff0653', 7), -- Fictional successor to Mulan from Disney's Mulan
('883c91c8-0951-4f8c-88d3-9a421adfa7a0', 7), -- Fictional successor to Lara Croft from Tomb Raider
('2b8d40e4-6401-45b7-bd8a-e0b3567ffdef', 7), -- Fictional successor to Guan Yu from Chinese history and folklore
('a61d4a4e-02c1-4710-9dc1-6061d0ef39ac', 7), -- Fictional successor to Aloy from Horizon Zero Dawn
('5ef8eb91-6287-47a6-bc29-5a39cc27b110', 8), -- Desmond Doss, real-life hero and conscientious objector in World War II
('1b304cec-6df6-4722-8638-8dd4280da77d', 8), -- Mahatma Gandhi, leader of the Indian independence movement and advocate for nonviolent resistance
('268fbf42-b13d-4d6b-bef9-2b2738b82e8c', 8), -- Fictional successor to Aragorn from Lord of the Rings
('92df2baf-0515-4d49-a9a0-1771548ac468', 8), -- Fictional successor to Katara from Avatar: The Last Airbender
('7a961d84-e3d9-4c78-9461-5db1c2dce9dc', 8), -- Fictional successor to Wonder Woman from DC Comics
('bda2c18c-5627-4874-84b0-8f1068b5f214', 8), -- Nelson Mandela, anti-apartheid revolutionary and former President of South Africa
('f69bf3b7-6b6c-49cb-b93d-fb4680d4bf0b', 8), -- Malala Yousafzai, Pakistani activist for female education and the youngest Nobel Prize laureate
('b2dcd9c1-d5ff-4ef1-8360-c6e0df8713ac', 8), -- Fictional character Sokka from Avatar: The Last Airbender
('c573a236-9f4e-4f88-85c7-b3aeeb43895b', 8), -- Fictional lion Aslan from The Chronicles of Narnia
('d895af07-2504-4c7e-b39d-07fcbe58b754', 8), -- Nelson Mandela, British admiral known for his service during the Napoleonic Wars
('29c46f07-0b40-409f-bc51-0cde4a9d4ab7', 8), -- Rosa Parks, civil rights activist known for her pivotal role in the Montgomery bus boycott
('d14061d5-1ad7-4c38-a6a5-264b548a2b2d', 8), -- Harriet Tubman, abolitionist and political activist known for her work as a "conductor" on the Underground Railroad
('f914937f-3d22-4822-98ec-6d5a9f0f2a4e', 8), -- Fictional successor to Éowyn from Lord of the Rings
('9a22720b-6ee3-4e09-8d3a-ff46b3eb1325', 8), -- Fictional successor to Galadriel from Lord of the Rings
('5dd8e422-8f5f-47d8-9a01-d7e62cf33ac1', 8), -- Fictional successor to Beowulf from the epic poem
('8e429a14-50ed-4c3d-8581-5d0e27654f3e', 8), -- Fictional successor to Joan of Arc from history and legend
('46d5ec24-5973-4cc2-8a88-620cd7a5a61c', 8), -- Fictional successor to Robin Hood from various folklore and adaptations
('fa63e28b-124e-40f1-af1b-4dcd82ff0653', 8), -- Fictional successor to Mulan from Disney's Mulan
('883c91c8-0951-4f8c-88d3-9a421adfa7a0', 8), -- Fictional successor to Lara Croft from Tomb Raider
('2b8d40e4-6401-45b7-bd8a-e0b3567ffdef', 8), -- Fictional successor to Guan Yu from Chinese history and folklore
('a61d4a4e-02c1-4710-9dc1-6061d0ef39ac', 8), -- Fictional successor to Aloy from Horizon Zero Dawn
('5ef8eb91-6287-47a6-bc29-5a39cc27b110', 9), -- Desmond Doss, real-life hero and conscientious objector in World War II
('1b304cec-6df6-4722-8638-8dd4280da77d', 9), -- Mahatma Gandhi, leader of the Indian independence movement and advocate for nonviolent resistance
('268fbf42-b13d-4d6b-bef9-2b2738b82e8c', 9), -- Fictional successor to Aragorn from Lord of the Rings
('92df2baf-0515-4d49-a9a0-1771548ac468', 9), -- Fictional successor to Katara from Avatar: The Last Airbender
('7a961d84-e3d9-4c78-9461-5db1c2dce9dc', 9), -- Fictional successor to Wonder Woman from DC Comics
('bda2c18c-5627-4874-84b0-8f1068b5f214', 9), -- Nelson Mandela, anti-apartheid revolutionary and former President of South Africa
('f69bf3b7-6b6c-49cb-b93d-fb4680d4bf0b', 9), -- Malala Yousafzai, Pakistani activist for female education and the youngest Nobel Prize laureate
('b2dcd9c1-d5ff-4ef1-8360-c6e0df8713ac', 9), -- Fictional character Sokka from Avatar: The Last Airbender
('c573a236-9f4e-4f88-85c7-b3aeeb43895b', 9), -- Fictional lion Aslan from The Chronicles of Narnia
('d895af07-2504-4c7e-b39d-07fcbe58b754', 9), -- Nelson Mandela, British admiral known for his service during the Napoleonic Wars
('29c46f07-0b40-409f-bc51-0cde4a9d4ab7', 9), -- Rosa Parks, civil rights activist known for her pivotal role in the Montgomery bus boycott
('d14061d5-1ad7-4c38-a6a5-264b548a2b2d', 9), -- Harriet Tubman, abolitionist and political activist known for her work as a "conductor" on the Underground Railroad
('f914937f-3d22-4822-98ec-6d5a9f0f2a4e', 9), -- Fictional successor to Éowyn from Lord of the Rings
('9a22720b-6ee3-4e09-8d3a-ff46b3eb1325', 9), -- Fictional successor to Galadriel from Lord of the Rings
('5dd8e422-8f5f-47d8-9a01-d7e62cf33ac1', 9), -- Fictional successor to Beowulf from the epic poem
('8e429a14-50ed-4c3d-8581-5d0e27654f3e', 9), -- Fictional successor to Joan of Arc from history and legend
('46d5ec24-5973-4cc2-8a88-620cd7a5a61c', 9), -- Fictional successor to Robin Hood from various folklore and adaptations
('fa63e28b-124e-40f1-af1b-4dcd82ff0653', 9), -- Fictional successor to Mulan from Disney's Mulan
('883c91c8-0951-4f8c-88d3-9a421adfa7a0', 9), -- Fictional successor to Lara Croft from Tomb Raider
('2b8d40e4-6401-45b7-bd8a-e0b3567ffdef', 9), -- Fictional successor to Guan Yu from Chinese history and folklore
('a61d4a4e-02c1-4710-9dc1-6061d0ef39ac', 9), -- Fictional successor to Aloy from Horizon Zero Dawn
('5ef8eb91-6287-47a6-bc29-5a39cc27b110', 10), -- Desmond Doss, real-life hero and conscientious objector in World War II
('1b304cec-6df6-4722-8638-8dd4280da77d', 10), -- Mahatma Gandhi, leader of the Indian independence movement and advocate for nonviolent resistance
('268fbf42-b13d-4d6b-bef9-2b2738b82e8c', 10), -- Fictional successor to Aragorn from Lord of the Rings
('92df2baf-0515-4d49-a9a0-1771548ac468', 10), -- Fictional successor to Katara from Avatar: The Last Airbender
('7a961d84-e3d9-4c78-9461-5db1c2dce9dc', 10), -- Fictional successor to Wonder Woman from DC Comics
('bda2c18c-5627-4874-84b0-8f1068b5f214', 10), -- Nelson Mandela, anti-apartheid revolutionary and former President of South Africa
('f69bf3b7-6b6c-49cb-b93d-fb4680d4bf0b', 10), -- Malala Yousafzai, Pakistani activist for female education and the youngest Nobel Prize laureate
('b2dcd9c1-d5ff-4ef1-8360-c6e0df8713ac', 10), -- Fictional character Sokka from Avatar: The Last Airbender
('c573a236-9f4e-4f88-85c7-b3aeeb43895b', 10), -- Fictional lion Aslan from The Chronicles of Narnia
('d895af07-2504-4c7e-b39d-07fcbe58b754', 10), -- Nelson Mandela, British admiral known for his service during the Napoleonic Wars
('29c46f07-0b40-409f-bc51-0cde4a9d4ab7', 10), -- Rosa Parks, civil rights activist known for her pivotal role in the Montgomery bus boycott
('d14061d5-1ad7-4c38-a6a5-264b548a2b2d', 10), -- Harriet Tubman, abolitionist and political activist known for her work as a "conductor" on the Underground Railroad
('f914937f-3d22-4822-98ec-6d5a9f0f2a4e', 10), -- Fictional successor to Éowyn from Lord of the Rings
('9a22720b-6ee3-4e09-8d3a-ff46b3eb1325', 10), -- Fictional successor to Galadriel from Lord of the Rings
('5dd8e422-8f5f-47d8-9a01-d7e62cf33ac1', 10), -- Fictional successor to Beowulf from the epic poem
('8e429a14-50ed-4c3d-8581-5d0e27654f3e', 10), -- Fictional successor to Joan of Arc from history and legend
('46d5ec24-5973-4cc2-8a88-620cd7a5a61c', 10), -- Fictional successor to Robin Hood from various folklore and adaptations
('fa63e28b-124e-40f1-af1b-4dcd82ff0653', 10), -- Fictional successor to Mulan from Disney's Mulan
('883c91c8-0951-4f8c-88d3-9a421adfa7a0', 10), -- Fictional successor to Lara Croft from Tomb Raider
('2b8d40e4-6401-45b7-bd8a-e0b3567ffdef', 10), -- Fictional successor to Guan Yu from Chinese history and folklore
('a61d4a4e-02c1-4710-9dc1-6061d0ef39ac', 10), -- Fictional successor to Aloy from Horizon Zero Dawn
('5ef8eb91-6287-47a6-bc29-5a39cc27b110', 11), -- Desmond Doss, real-life hero and conscientious objector in World War II
('1b304cec-6df6-4722-8638-8dd4280da77d', 11), -- Mahatma Gandhi, leader of the Indian independence movement and advocate for nonviolent resistance
('268fbf42-b13d-4d6b-bef9-2b2738b82e8c', 11), -- Fictional successor to Aragorn from Lord of the Rings
('92df2baf-0515-4d49-a9a0-1771548ac468', 11), -- Fictional successor to Katara from Avatar: The Last Airbender
('7a961d84-e3d9-4c78-9461-5db1c2dce9dc', 11), -- Fictional successor to Wonder Woman from DC Comics
('bda2c18c-5627-4874-84b0-8f1068b5f214', 11), -- Nelson Mandela, anti-apartheid revolutionary and former President of South Africa
('f69bf3b7-6b6c-49cb-b93d-fb4680d4bf0b', 11), -- Malala Yousafzai, Pakistani activist for female education and the youngest Nobel Prize laureate
('b2dcd9c1-d5ff-4ef1-8360-c6e0df8713ac', 11), -- Fictional character Sokka from Avatar: The Last Airbender
('c573a236-9f4e-4f88-85c7-b3aeeb43895b', 11), -- Fictional lion Aslan from The Chronicles of Narnia
('d895af07-2504-4c7e-b39d-07fcbe58b754', 11), -- Nelson Mandela, British admiral known for his service during the Napoleonic Wars
('29c46f07-0b40-409f-bc51-0cde4a9d4ab7', 11), -- Rosa Parks, civil rights activist known for her pivotal role in the Montgomery bus boycott
('d14061d5-1ad7-4c38-a6a5-264b548a2b2d', 11), -- Harriet Tubman, abolitionist and political activist known for her work as a "conductor" on the Underground Railroad
('f914937f-3d22-4822-98ec-6d5a9f0f2a4e', 11), -- Fictional successor to Éowyn from Lord of the Rings
('9a22720b-6ee3-4e09-8d3a-ff46b3eb1325', 11), -- Fictional successor to Galadriel from Lord of the Rings
('5dd8e422-8f5f-47d8-9a01-d7e62cf33ac1', 11), -- Fictional successor to Beowulf from the epic poem
('8e429a14-50ed-4c3d-8581-5d0e27654f3e', 11), -- Fictional successor to Joan of Arc from history and legend
('46d5ec24-5973-4cc2-8a88-620cd7a5a61c', 11), -- Fictional successor to Robin Hood from various folklore and adaptations
('fa63e28b-124e-40f1-af1b-4dcd82ff0653', 11), -- Fictional successor to Mulan from Disney's Mulan
('883c91c8-0951-4f8c-88d3-9a421adfa7a0', 11), -- Fictional successor to Lara Croft from Tomb Raider
('2b8d40e4-6401-45b7-bd8a-e0b3567ffdef', 11), -- Fictional successor to Guan Yu from Chinese history and folklore
('a61d4a4e-02c1-4710-9dc1-6061d0ef39ac', 11), -- Fictional successor to Aloy from Horizon Zero Dawn
('5ef8eb91-6287-47a6-bc29-5a39cc27b110', 12), -- Desmond Doss, real-life hero and conscientious objector in World War II
('1b304cec-6df6-4722-8638-8dd4280da77d', 12), -- Mahatma Gandhi, leader of the Indian independence movement and advocate for nonviolent resistance
('268fbf42-b13d-4d6b-bef9-2b2738b82e8c', 12), -- Fictional successor to Aragorn from Lord of the Rings
('92df2baf-0515-4d49-a9a0-1771548ac468', 12), -- Fictional successor to Katara from Avatar: The Last Airbender
('7a961d84-e3d9-4c78-9461-5db1c2dce9dc', 12), -- Fictional successor to Wonder Woman from DC Comics
('bda2c18c-5627-4874-84b0-8f1068b5f214', 12), -- Nelson Mandela, anti-apartheid revolutionary and former President of South Africa
('f69bf3b7-6b6c-49cb-b93d-fb4680d4bf0b', 12), -- Malala Yousafzai, Pakistani activist for female education and the youngest Nobel Prize laureate
('b2dcd9c1-d5ff-4ef1-8360-c6e0df8713ac', 12), -- Fictional character Sokka from Avatar: The Last Airbender
('c573a236-9f4e-4f88-85c7-b3aeeb43895b', 12), -- Fictional lion Aslan from The Chronicles of Narnia
('d895af07-2504-4c7e-b39d-07fcbe58b754', 12), -- Nelson Mandela, British admiral known for his service during the Napoleonic Wars
('29c46f07-0b40-409f-bc51-0cde4a9d4ab7', 12), -- Rosa Parks, civil rights activist known for her pivotal role in the Montgomery bus boycott
('d14061d5-1ad7-4c38-a6a5-264b548a2b2d', 12), -- Harriet Tubman, abolitionist and political activist known for her work as a "conductor" on the Underground Railroad
('f914937f-3d22-4822-98ec-6d5a9f0f2a4e', 12), -- Fictional successor to Éowyn from Lord of the Rings
('9a22720b-6ee3-4e09-8d3a-ff46b3eb1325', 12), -- Fictional successor to Galadriel from Lord of the Rings
('5dd8e422-8f5f-47d8-9a01-d7e62cf33ac1', 12), -- Fictional successor to Beowulf from the epic poem
('8e429a14-50ed-4c3d-8581-5d0e27654f3e', 12), -- Fictional successor to Joan of Arc from history and legend
('46d5ec24-5973-4cc2-8a88-620cd7a5a61c', 12), -- Fictional successor to Robin Hood from various folklore and adaptations
('fa63e28b-124e-40f1-af1b-4dcd82ff0653', 12), -- Fictional successor to Mulan from Disney's Mulan
('883c91c8-0951-4f8c-88d3-9a421adfa7a0', 12), -- Fictional successor to Lara Croft from Tomb Raider
('2b8d40e4-6401-45b7-bd8a-e0b3567ffdef', 12), -- Fictional successor to Guan Yu from Chinese history and folklore
('a61d4a4e-02c1-4710-9dc1-6061d0ef39ac', 12), -- Fictional successor to Aloy from Horizon Zero Dawn
('5ef8eb91-6287-47a6-bc29-5a39cc27b110', 13), -- Desmond Doss, real-life hero and conscientious objector in World War II
('1b304cec-6df6-4722-8638-8dd4280da77d', 13), -- Mahatma Gandhi, leader of the Indian independence movement and advocate for nonviolent resistance
('268fbf42-b13d-4d6b-bef9-2b2738b82e8c', 13), -- Fictional successor to Aragorn from Lord of the Rings
('92df2baf-0515-4d49-a9a0-1771548ac468', 13), -- Fictional successor to Katara from Avatar: The Last Airbender
('7a961d84-e3d9-4c78-9461-5db1c2dce9dc', 13), -- Fictional successor to Wonder Woman from DC Comics
('bda2c18c-5627-4874-84b0-8f1068b5f214', 13), -- Nelson Mandela, anti-apartheid revolutionary and former President of South Africa
('f69bf3b7-6b6c-49cb-b93d-fb4680d4bf0b', 13), -- Malala Yousafzai, Pakistani activist for female education and the youngest Nobel Prize laureate
('b2dcd9c1-d5ff-4ef1-8360-c6e0df8713ac', 13), -- Fictional character Sokka from Avatar: The Last Airbender
('c573a236-9f4e-4f88-85c7-b3aeeb43895b', 13), -- Fictional lion Aslan from The Chronicles of Narnia
('d895af07-2504-4c7e-b39d-07fcbe58b754', 13), -- Nelson Mandela, British admiral known for his service during the Napoleonic Wars
('29c46f07-0b40-409f-bc51-0cde4a9d4ab7', 13), -- Rosa Parks, civil rights activist known for her pivotal role in the Montgomery bus boycott
('d14061d5-1ad7-4c38-a6a5-264b548a2b2d', 13), -- Harriet Tubman, abolitionist and political activist known for her work as a "conductor" on the Underground Railroad
('f914937f-3d22-4822-98ec-6d5a9f0f2a4e', 13), -- Fictional successor to Éowyn from Lord of the Rings
('9a22720b-6ee3-4e09-8d3a-ff46b3eb1325', 13), -- Fictional successor to Galadriel from Lord of the Rings
('5dd8e422-8f5f-47d8-9a01-d7e62cf33ac1', 13), -- Fictional successor to Beowulf from the epic poem
('8e429a14-50ed-4c3d-8581-5d0e27654f3e', 13), -- Fictional successor to Joan of Arc from history and legend
('46d5ec24-5973-4cc2-8a88-620cd7a5a61c', 13), -- Fictional successor to Robin Hood from various folklore and adaptations
('fa63e28b-124e-40f1-af1b-4dcd82ff0653', 13), -- Fictional successor to Mulan from Disney's Mulan
('883c91c8-0951-4f8c-88d3-9a421adfa7a0', 13), -- Fictional successor to Lara Croft from Tomb Raider
('2b8d40e4-6401-45b7-bd8a-e0b3567ffdef', 13), -- Fictional successor to Guan Yu from Chinese history and folklore
('a61d4a4e-02c1-4710-9dc1-6061d0ef39ac', 13), -- Fictional successor to Aloy from Horizon Zero Dawn
('5ef8eb91-6287-47a6-bc29-5a39cc27b110', 14), -- Desmond Doss, real-life hero and conscientious objector in World War II
('1b304cec-6df6-4722-8638-8dd4280da77d', 14), -- Mahatma Gandhi, leader of the Indian independence movement and advocate for nonviolent resistance
('268fbf42-b13d-4d6b-bef9-2b2738b82e8c', 14), -- Fictional successor to Aragorn from Lord of the Rings
('92df2baf-0515-4d49-a9a0-1771548ac468', 14), -- Fictional successor to Katara from Avatar: The Last Airbender
('7a961d84-e3d9-4c78-9461-5db1c2dce9dc', 14), -- Fictional successor to Wonder Woman from DC Comics
('bda2c18c-5627-4874-84b0-8f1068b5f214', 14), -- Nelson Mandela, anti-apartheid revolutionary and former President of South Africa
('f69bf3b7-6b6c-49cb-b93d-fb4680d4bf0b', 14), -- Malala Yousafzai, Pakistani activist for female education and the youngest Nobel Prize laureate
('b2dcd9c1-d5ff-4ef1-8360-c6e0df8713ac', 14), -- Fictional character Sokka from Avatar: The Last Airbender
('c573a236-9f4e-4f88-85c7-b3aeeb43895b', 14), -- Fictional lion Aslan from The Chronicles of Narnia
('d895af07-2504-4c7e-b39d-07fcbe58b754', 14), -- Nelson Mandela, British admiral known for his service during the Napoleonic Wars
('29c46f07-0b40-409f-bc51-0cde4a9d4ab7', 14), -- Rosa Parks, civil rights activist known for her pivotal role in the Montgomery bus boycott
('d14061d5-1ad7-4c38-a6a5-264b548a2b2d', 14), -- Harriet Tubman, abolitionist and political activist known for her work as a "conductor" on the Underground Railroad
('f914937f-3d22-4822-98ec-6d5a9f0f2a4e', 14), -- Fictional successor to Éowyn from Lord of the Rings
('9a22720b-6ee3-4e09-8d3a-ff46b3eb1325', 14), -- Fictional successor to Galadriel from Lord of the Rings
('5dd8e422-8f5f-47d8-9a01-d7e62cf33ac1', 14), -- Fictional successor to Beowulf from the epic poem
('8e429a14-50ed-4c3d-8581-5d0e27654f3e', 14), -- Fictional successor to Joan of Arc from history and legend
('46d5ec24-5973-4cc2-8a88-620cd7a5a61c', 14), -- Fictional successor to Robin Hood from various folklore and adaptations
('fa63e28b-124e-40f1-af1b-4dcd82ff0653', 14), -- Fictional successor to Mulan from Disney's Mulan
('883c91c8-0951-4f8c-88d3-9a421adfa7a0', 14), -- Fictional successor to Lara Croft from Tomb Raider
('2b8d40e4-6401-45b7-bd8a-e0b3567ffdef', 14), -- Fictional successor to Guan Yu from Chinese history and folklore
('a61d4a4e-02c1-4710-9dc1-6061d0ef39ac', 14); -- Fictional successor to Aloy from Horizon Zero Dawn