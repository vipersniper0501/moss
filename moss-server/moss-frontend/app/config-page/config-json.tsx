'use client';
import useSWR from 'swr';
import React, {useEffect} from 'react';
import { useState } from 'react';
import styles from './config-json.module.scss';
import {MossData, jsonToMossData} from './config-api';
import JSONPretty from 'react-json-pretty';

type WrapperProps = {
    os: React.ReactNode;
};

type MossWrapperProps = {
    data: MossData | string;
}

function ConfigMossDataForm(props: MossWrapperProps) {
    if (typeof props.data === "string") {
        return <></>;
    }

    return (
        <form>
            <label>Approved Files:</label>
        </form>
       );

}

export default function ConfigJsonPortal(props: WrapperProps) {

    const [mossdata, setMossdata] = useState<MossData | string>("No data");

    const {data, error, isLoading} = useSWR('http://127.0.0.1:4224/api/v1/config/' + props.os,
                                            async (url) => {
                                                return fetch(url, {method: 'GET'})
                                                .then(res => res.json());
                                            });

    useEffect(() => {
        if (data != undefined && data != "No data") {
            setMossdata(jsonToMossData(data));
        }
    }, [data]);


    if (error) return <p>Error occurred fetching config data for {props.os}</p>;
    if (isLoading) return <p>Loading data...</p>;
    return (
        <div>
            <ConfigMossDataForm data={mossdata}></ConfigMossDataForm>
            <p>Json Preview:</p>
            <JSONPretty className={styles.json} data={mossdata}></JSONPretty>
        </div>
    );
}
