'use client';
import { useEffect, useState } from 'react';

import styles from './styles/page.module.scss'

async function getTeams() {
    try {
        const request = await fetch(
            "http://127.0.0.1:4224/api/v1/teams",
            {method: "GET"}
        );

        const data = await request.json();
        console.log(data);
        return data;
    } catch (err) {
        console.log(err);
        throw err;
    }
}


interface Team {
    team_id: number;
    name: string;
}

function TeamsList() {
    const [teams, setTeams] = useState<Team[] | undefined>(undefined);

    useEffect(() => {
        async function fetchTeams() {
            try {
                const data: Team[] = await getTeams();
                setTeams(data);

            } catch (error) {
                console.error(error);
            }
        }

        fetchTeams();

    }, []);

    if (teams == undefined) {
        return <p>Loading teams...</p>;
    }
    return (
        <div>
            {teams.map((val, index) => (
                    <div key={index} className={styles.team}>
                        <p>{val.name}</p>
                    </div>
            ))}
        </div>

    );
}


export default function Home() {
    console.log("Test log");
    return (
        <>
            <div className={styles.content}>
                <h1>Moss</h1>
                <div className={styles.cardsColumn}>
                    <div className={styles.card}>
                        <p>Card 1</p>
                    </div>
                    <div className={styles.card}>
                        <p>Card 2</p>
                    </div>
                    <div className={styles.card}>
                        <p>Card 3</p>
                    </div>
                </div>

                <div className={styles.teamsColumn}>
                    <TeamsList />
                </div>
            </div>
        </>
    );
}
