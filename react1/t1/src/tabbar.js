import React, { PureComponent } from 'react'
import ReactDOM from 'react-dom'

import 'antd-mobile/dist/antd-mobile.css';
import { TabBar } from 'antd-mobile'

class Main extends PureComponent {
    constructor(props) {
        super(props)
        this.state = {
            selectedTab: 'monitor',
        }
    }

    render() {
        const { selectedTab } = this.state

        return (
            <div style={{height: '640px'}}>
                <TabBar>
                    <TabBar.Item
                        title="监控"
                        icon={<div style={{
                            width: '22px',
                            height: '22px',
                            background: 'url(home.svg) center center /  21px 21px no-repeat' }}
                        />}
                        selectedIcon={<div style={{
                            width: '22px',
                            height: '22px',
                            background: 'url(home.svg) center center /  21px 21px no-repeat' }}
                        />}
                        selected={ selectedTab === 'monitor'}
                        onPress={() => {
                            this.setState({
                            selectedTab: 'monitor',
                            });
                        }}
                    >

                    </TabBar.Item>
                    
                    <TabBar.Item
                        title="报警"
                        icon={<div style={{
                            width: '22px',
                            height: '22px',
                            background: 'url(home.svg) center center /  21px 21px no-repeat' }}
                        />}
                        selectedIcon={<div style={{
                            width: '22px',
                            height: '22px',
                            background: 'url(home.svg) center center /  21px 21px no-repeat' }}
                        />}
                        selected={ selectedTab === 'alarm'}
                        onPress={() => {
                            this.setState({
                            selectedTab: 'alarm',
                            });
                        }}
                    >

                    </TabBar.Item>

                    <TabBar.Item
                        title="区域"
                        icon={<div style={{
                            width: '22px',
                            height: '22px',
                            background: 'url(home.svg) center center /  21px 21px no-repeat' }}
                        />}
                        selectedIcon={<div style={{
                            width: '22px',
                            height: '22px',
                            background: 'url(home.svg) center center /  21px 21px no-repeat' }}
                        />}
                        selected={ selectedTab === 'region'}
                        onPress={() => {
                            this.setState({
                            selectedTab: 'region',
                            });
                        }}
                    >

                    </TabBar.Item>

                    <TabBar.Item
                        title="我的"
                        icon={<div style={{
                            width: '22px',
                            height: '22px',
                            background: 'url(home.svg) center center /  21px 21px no-repeat' }}
                        />}
                        selectedIcon={<div style={{
                            width: '22px',
                            height: '22px',
                            background: 'url(home.svg) center center /  21px 21px no-repeat' }}
                        />}
                        selected={ selectedTab === 'my'}
                        onPress={() => {
                            this.setState({
                            selectedTab: 'my',
                            });
                        }}
                    >

                    </TabBar.Item>
                </TabBar>
            </div>
        )
    }
}

ReactDOM.render(<Main />, document.getElementById('root'))
