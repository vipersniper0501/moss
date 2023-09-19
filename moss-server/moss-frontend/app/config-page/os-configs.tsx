'use client';
import useSWR from 'swr';

export default function OperatingSystemConfigs() {

    const {data, error, isLoading} = useSWR('http://127.0.0.1:4224/api/v1/os',
                                            async (url) => {
                                                return fetch(url, {method: 'GET'})
                                                .then(res => res.json());
                                            });

    if (error) return <p>Error occurred fetching operating systems</p>;
    if (isLoading) return <p>Loading data...</p>;


    // console.log(data.operating_systems[0]);
    // console.log(data);
    return (
        <div>
        {data.operating_systems.map((val: string, index: number) => (
                <div key={index}>
                    <p>{val}</p>
                </div>
            ))}
        </div>
    );
}
