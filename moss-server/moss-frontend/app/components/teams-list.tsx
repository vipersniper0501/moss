import useSWR from 'swr';
import styles from '../styles/page.module.scss';

interface Team {
    team_id: number;
    name: string;
}

export function TeamsList() {
    const {data, error, isLoading} =  useSWR('https://' + location.hostname + ':4224/api/v1/teams',
                                            async (url) => {
                                                return fetch(url, {method: 'GET'})
                                                .then(res => res.json());
                                            });

    if (error) return <p>Error occurred fetching teams. Are you sure both the
                        backend and docker database are running?</p>;

    if (isLoading) {
        return <p>Loading teams...</p>;
    }
    return (
        <div>
        {data.map((val: Team, index: number) => (
                <div key={index} className={styles.team}>
                    <p>{val.name}</p>
                </div>
        ))}
        </div>

    );
}
