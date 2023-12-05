'use client';
import React from 'react';
import styles from './config-json.module.scss';
import {MossData, MossFileData} from './config-api';
// import debounce from 'lodash';

type MossWrapperProps = {
    data: MossData;
    changeState: any;
    system: string;
}

/**
 * @param props
 *  * data: MossData object
 *  * changeState: closure with the ability to update the react state mossdata
 * @returns an html form that allows you to edit mossdata
 */
export function ConfigMossDataForm(props: MossWrapperProps) {

    const updateDatabase = (updatedData: MossData) => {
        console.log(updatedData);
        var myHeaders = new Headers();
        myHeaders.append("Content-Type", "application/json");

        fetch('https://' + location.hostname + ':4224/api/v1/config/' + props.system,
            {
                method: 'PUT',
                headers: myHeaders,
                body: JSON.stringify(updatedData)
            })
            .then(res => res.text())
            .catch(error => console.log('error', error));
    };

    return (
        <form>
            <label>Approved Files:</label>
            <ul>
            {
            props.data.approved_files.map((val: MossFileData, index) => (
                    <div key={index} className={styles.editorBox}>
                        <label>Name: </label>
                        <input 
                            type="text" 
                            value={val.name} 
                            onChange={(e) => {
                                props.changeState(() => {
                                        let updatedData: MossData = {...props.data};
                                        updatedData.approved_files[index].name = e.target.value;
                                        return updatedData;
                                    });
                        }}
                        ></input>
                        <br></br>
                        <label>Location: </label>
                        <input 
                            type="text" 
                            value={val.location} 
                            onChange={(e) => {
                                props.changeState(() => {
                                        let updatedData: MossData = {...props.data};
                                        updatedData.approved_files[index].location = e.target.value;
                                        return updatedData;
                                    });
                        }}
                        ></input><br></br>
                        <input 
                            type="button" 
                            value="Delete" 
                            className={styles.deleteButton}
                            onClick={() => {
                                props.changeState(() => {
                                    let updatedData: MossData = {...props.data};
                                    updatedData.approved_files.splice(index, 1);
                                    return updatedData;
                                    });
                        }}></input>
                    </div>
                ))
            }
                <input type="button" value="Add File" onClick={() => {
                    props.changeState(() => {
                            let updatedData: MossData = {...props.data};
                            let newFile: MossFileData = {name: "", location: ""};
                            updatedData.approved_files.push(newFile);
                            return updatedData;
                        })
                }}></input>
            </ul>
            <label>Invalid Files:</label>
            <ul>
            {
            props.data.invalid_files.map((val: MossFileData, index) => (
                    <div key={index} className={styles.editorBox}>
                    <label>Name: </label>
                    <input 
                        type="text" 
                        value={val.name} 
                        onChange={(e) => {
                            props.changeState(() => {
                                    let updatedData: MossData = {...props.data};
                                    updatedData.invalid_files[index].name = e.target.value;
                                    return updatedData;
                                });
                    }}
                    ></input> 
                    <br></br>
                    <label>Location: </label>
                    <input 
                        type="text" 
                        value={val.location} 
                        onChange={(e) => {
                            props.changeState(() => {
                                    let updatedData: MossData = {...props.data};
                                    updatedData.invalid_files[index].location = e.target.value;
                                    return updatedData;
                                });
                    }}
                    ></input><br></br>
                    <input 
                        type="button" 
                        value="Delete" 
                        className={styles.deleteButton}
                        onClick={() => {
                            props.changeState(() => {
                                let updatedData: MossData = {...props.data};
                                updatedData.invalid_files.splice(index, 1);
                                return updatedData;
                                });
                    }}></input>
                    </div>
                ))

            }
                <input type="button" value="Add File" onClick={() => {
                    props.changeState(() => {
                            let updatedData: MossData = {...props.data};
                            let newFile: MossFileData = {name: "", location: ""};
                            updatedData.invalid_files.push(newFile);
                            return updatedData;
                        })
                }}></input>
            </ul>
            <label>Valid Users:</label>
            <ul>
            {
                props.data.valid_users.map((val: string, index: number) => (
                    <div key={index} className={styles.editorBox}>
                            <label>Name: </label>
                            <input 
                                type="text" 
                                value={val}
                                onChange={(e) => {
                                props.changeState(() => {
                                        let updatedData: MossData = {...props.data};
                                        updatedData.valid_users[index] = e.target.value;
                                        return updatedData;
                                    });
                                }}
                                ></input>
                                <br></br>
                    <input 
                        type="button" 
                        value="Delete" 
                        className={styles.deleteButton}
                        onClick={() => {
                            props.changeState(() => {
                                let updatedData: MossData = {...props.data};
                                updatedData.valid_users.splice(index, 1);
                                return updatedData;
                                });
                    }}></input>
                    </div>
                ))
            }
                <input type="button" value="Add User" onClick={() => {
                    props.changeState(() => {
                            let updatedData: MossData = {...props.data};
                            let newUser: string = "";
                            updatedData.valid_users.push(newUser);
                            return updatedData;
                        })
                }}></input>
            </ul>
            <label>Invalid Users:</label>
            <ul>
            {
                props.data.invalid_users.map((val: string, index: number) => (
                    <div key={index} className={styles.editorBox}>
                            <label>Name: </label>
                            <input 
                                type="text" 
                                value={val}
                                onChange={(e) => {
                                props.changeState(() => {
                                        let updatedData: MossData = {...props.data};
                                        updatedData.invalid_users[index] = e.target.value;
                                        return updatedData;
                                    });
                                }}
                            ></input>
                            <br></br>
                    <input 
                        type="button" 
                        value="Delete" 
                        className={styles.deleteButton}
                        onClick={() => {
                            props.changeState(() => {
                                let updatedData: MossData = {...props.data};
                                updatedData.invalid_users.splice(index, 1);
                                return updatedData;
                                });
                    }}></input>
                    </div>
                ))
            }
                <input type="button" value="Add User" onClick={() => {
                    props.changeState(() => {
                            let updatedData: MossData = {...props.data};
                            let newUser: string = "";
                            updatedData.invalid_users.push(newUser);
                            return updatedData;
                        })
                }}></input>
            </ul>
            <button type="button" onClick={(_e) => {updateDatabase(props.data)}}>Save</button>
        </form>
       );

}
