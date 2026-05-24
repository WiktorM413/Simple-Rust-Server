CREATE TABLE `users`
(
	`id`       int          NOT NULL,
	`username` varchar(255) NOT NULL,
	`age`      int          NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=latin1 COLLATE=latin1_swedish_ci;

ALTER TABLE `users`
	ADD PRIMARY KEY (`id`),
	ADD UNIQUE KEY `username` (`username`);

ALTER TABLE `users`
	MODIFY `id` int NOT NULL AUTO_INCREMENT;