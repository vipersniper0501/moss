
import Link from "next/link";
import styles from "../styles/navbar.module.scss";

export function AdminNavbar() {
    return (
        <div className={styles.navHorizontal}>
            <ul>
                <li><Link className={styles.active} href="/">Home</Link></li>
                <li><Link href="/config-page">Configurations</Link></li>

                <li style={{float: "right", display: "block", 
                            color: "white", textAlign: "center",
                            padding: "14px 16px", textDecoration: "none"
                            }}>Moss Admin Dashboard</li>
            </ul>

        </div>
    )

}
