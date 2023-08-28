CREATE TABLE IF NOT EXISTS Teams (
    team_id INT PRIMARY KEY,
    team_name VARCHAR(100)
);

CREATE TABLE IF NOT EXISTS Configurations (
    configuration_id SERIAL PRIMARY KEY,
    team_id INT REFERENCES Teams(team_id),
    operating_system VARCHAR(50),
    configuration_data JSON
);

CREATE TABLE IF NOT EXISTS Results (
    result_id SERIAL PRIMARY KEY,
    team_id INT REFERENCES Teams(team_id),
    operating_system VARCHAR(50),
    result_data JSON
)
