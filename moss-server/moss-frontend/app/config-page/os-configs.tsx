'use client';
import useSWR from 'swr';
import ConfigJsonPortal from './config-json';

export default function OperatingSystemConfigs() {

    const {data, error, isLoading} = useSWR('http://' + location.hostname + ':4224/api/v1/systems',
                                            async (url) => {
                                                return fetch(url, {method: 'GET'})
                                                .then(res => res.json());
                                            });

    if (error) return <p>Error occurred fetching operating systems</p>;
    if (isLoading) return <p>Loading data...</p>;

    let loaded_data: string[] = data.systems;
    let formatted_data = loaded_data.map((val: string, index: number) => (
        <div key={index}>
            <h3>{val}</h3>
            <ConfigJsonPortal system = {val} />
        </div>
    ));

    return (
        <div>
        {formatted_data}
        </div>
    );
}
