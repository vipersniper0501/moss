import styles from '../styles/config-page.module.scss';

import OperatingSystemConfigs from './os-configs';

export default function Config() {

    return (
        <div className={styles.content}>
            <h2>System Configs</h2>
            <OperatingSystemConfigs />
        </div>
    );
}
