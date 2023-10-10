'use client';
import useSWR from 'swr';
import React, {useEffect} from 'react';
import { useState } from 'react';
import styles from './config-json.module.scss';
import {MossData, MossFileData, jsonToMossData, createEmptyMossData} from './config-api';
import JSONPretty from 'react-json-pretty';

type WrapperProps = {
    os: React.ReactNode;
};

type MossWrapperProps = {
    data: MossData;
    changeState:any;
}

function ConfigMossDataForm(props: MossWrapperProps) {

    if (JSON.stringify(props.data) == JSON.stringify(createEmptyMossData())) {
        return <></>
    }

    return (
        <form>
            <label>Approved Files:</label>
            <ul>
            {
            props.data.approved_files.map((val: MossFileData, index) => (
                    <div key={index}>
                    <label>Name: </label>
                    <input 
                        type="text" 
                        value={val.name} 
                        onChange={(e) => {
                            // props.changeState(index, e.target.value);
                            props.changeState(() => {
                                    let updatedData: MossData = {...props.data};
                                    updatedData.approved_files[index].name = e.target.value;
                                    return updatedData;
                                });
                    }}
                    ></input>
                    <ul>
                        <label>Location: </label>
                        <input 
                            type="text" 
                            value={val.location} 
                            onChange={(e) => {
                                // props.changeState(index, e.target.value);
                                props.changeState(() => {
                                        let updatedData: MossData = {...props.data};
                                        updatedData.approved_files[index].location = e.target.value;
                                        return updatedData;
                                    });
                        }}
                        ></input>
                    </ul>
                    <br></br>
                    </div>
                ))
            }
            </ul>
            <label>Invalid Files:</label>
            <ul>
            {
            props.data.invalid_files.map((val: MossFileData, index) => (
                    <div key={index}>
                    <label>Name: </label>
                    <input 
                        type="text" 
                        value={val.name} 
                        onChange={(e) => {
                            // props.changeState(index, e.target.value);
                            props.changeState(() => {
                                    let updatedData: MossData = {...props.data};
                                    updatedData.invalid_files[index].name = e.target.value;
                                    return updatedData;
                                });
                    }}
                    ></input>
                    <ul>
                        <label>Location: </label>
                        <input 
                            type="text" 
                            value={val.location} 
                            onChange={(e) => {
                                // props.changeState(index, e.target.value);
                                props.changeState(() => {
                                        let updatedData: MossData = {...props.data};
                                        updatedData.invalid_files[index].location = e.target.value;
                                        return updatedData;
                                    });
                        }}
                        ></input>
                    </ul>
                    <br></br>
                    </div>
                ))

            }
            </ul>
        </form>
       );

}

export default function ConfigJsonPortal(props: WrapperProps) {

    const [mossdata, setMossdata] = useState<MossData>(createEmptyMossData());

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

    // const handleState = (index: number, val: string) => {
    const handleStateUpdate = (val: MossData) => {
        setMossdata(val);
    };

    if (error) return <p>Error occurred fetching config data for {props.os}</p>;
    if (isLoading) return <p>Loading data...</p>;
    return (
        <div>
            <ConfigMossDataForm data={mossdata} changeState={handleStateUpdate}></ConfigMossDataForm>
            <p>Json Preview:</p>
            <JSONPretty className={styles.json} data={mossdata}></JSONPretty>
        </div>
    );
}
