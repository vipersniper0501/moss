'use client';
import useSWR from 'swr';
import ConfigJsonPortal from './config-json';

export default function OperatingSystemConfigs() {

    const {data, error, isLoading} = useSWR('http://127.0.0.1:4224/api/v1/os',
                                            async (url) => {
                                                return fetch(url, {method: 'GET'})
                                                .then(res => res.json());
                                            });

    if (error) return <p>Error occurred fetching operating systems</p>;
    if (isLoading) return <p>Loading data...</p>;

    let loaded_data: string[] = data.operating_systems;
    let formatted_data = loaded_data.map((val: string, index: number) => (
        <div key={index}>
            <h3>{val}</h3>
            <ConfigJsonPortal os = {val} />
        </div>
    ));

    return (
        <div>
        {formatted_data}
        </div>
    );
}
