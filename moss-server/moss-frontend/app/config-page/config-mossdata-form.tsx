'use client';
import React from 'react';
import styles from './config-json.module.scss';
import {MossData, MossFileData, createEmptyMossData} from './config-api';

type MossWrapperProps = {
    data: MossData;
    changeState:any;
}

/**
 * @param props
 *  * data: MossData object
 *  * changeState: closure with the ability to update the react state mossdata
 * @returns an html form that allows you to edit mossdata
 */
export function ConfigMossDataForm(props: MossWrapperProps) {

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
