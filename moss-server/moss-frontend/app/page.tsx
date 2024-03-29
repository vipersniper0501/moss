'use client';
import { TeamsList } from './teams-list';
import styles from './styles/page.module.scss'




export default function Home() {

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
