'use client';
import useSWR from 'swr';
import React, {useEffect} from 'react';
import { useState } from 'react';
import styles from './config-json.module.scss';
import {MossData, jsonToMossData, createEmptyMossData} from './config-api';
import JSONPretty from 'react-json-pretty';
import { ConfigMossDataForm } from './config-mossdata-form';

type WrapperProps = {
    system: string;
};


/**
 * @param props - 
 *  * os: string with name of the operating system
 * @returns html with an editor for the mossdata object and a preview of the
 * object in json form.
 */
export default function ConfigJsonPortal(props: WrapperProps) {

    const [mossdata, setMossdata] = useState<MossData>(createEmptyMossData());

    const {data, error, isLoading} = useSWR('http://127.0.0.1:4224/api/v1/config/' + props.system,
                                            async (url) => {
                                                return fetch(url, {method: 'GET'})
                                                .then(res => res.json());
                                            });

    useEffect(() => {
        if (data != undefined && data != "No data") {
            setMossdata(jsonToMossData(data));
        }
    }, [data]);

    // const handleState = (index: number, val: string) => {
    const handleStateUpdate = (val: MossData) => {
        setMossdata(val);
    };

    if (error) return <p>Error occurred fetching config data for {props.system}</p>;
    if (isLoading) return <p>Loading data...</p>;
    return (
        <div className={styles.hostBox}>
            <ConfigMossDataForm data={mossdata} changeState={handleStateUpdate} system={props.system}></ConfigMossDataForm>
            <div className={styles.jsonView}>
                <p>Json Preview:</p>
                <JSONPretty className={styles.json} data={mossdata}></JSONPretty>
            </div>
        </div>
    );
}
