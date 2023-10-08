'use client';
import useSWR from 'swr';
import React from 'react';
import styles from './config-json.module.scss';
import {MossData} from './config-api';
import JSONPretty from 'react-json-pretty';

type WrapperProps = {
    os: React.ReactNode;
};

export default function ConfigJsonPortal(props: WrapperProps) {

    const {data, error, isLoading} = useSWR<MossData>('http://127.0.0.1:4224/api/v1/config/' + props.os,
    // const {data, error, isLoading} = useSWR('http://127.0.0.1:4224/api/v1/config/' + props.os,
                                            async (url) => {
                                                return fetch(url, {method: 'GET'})
                                                .then(res => res.json());
                                            });

    if (error) return <p>Error occurred fetching config data for {props.os}</p>;
    if (isLoading) return <p>Loading data...</p>;


    console.log(data);

    return (
        <div>
            <form>

            </form>
            <p>Json Preview:</p>
            <JSONPretty className={styles.json} data={data}></JSONPretty>
        </div>
    );
}
