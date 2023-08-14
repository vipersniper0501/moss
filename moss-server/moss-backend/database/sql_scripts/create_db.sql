CREATE TABLE IF NOT EXISTS Teams (
    TeamID SERIAL PRIMARY KEY,
    TeamName VARCHAR(100)
);

CREATE TABLE IF NOT EXISTS Configurations (
    ConfigurationID SERIAL PRIMARY KEY,
    TeamID INT REFERENCES Teams(TeamID),
    OperatingSystem VARCHAR(50),
    ConfigurationData JSON
);

CREATE TABLE IF NOT EXISTS Results (
    ResultID SERIAL PRIMARY KEY,
    TeamID INT REFERENCES Teams(TeamID),
    OperatingSystem VARCHAR(50),
    ResultData JSON
)
