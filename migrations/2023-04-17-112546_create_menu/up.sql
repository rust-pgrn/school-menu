CREATE TABLE menu ( 
	id SERIAL PRIMARY KEY,
	date DATE NOT NULL,
	meal TEXT NOT NULL,
	students TEXT[] NOT NULL,
	price MONEY NOT NULL
)
