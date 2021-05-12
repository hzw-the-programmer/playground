import DatePicker from './components/antd/DatePicker'
import HorizontalLoginForm from './components/antd/form/normform/HorizontalLoginForm'
import DownloadForm from './components/antd/form/normform/DownloadForm'
import PriceForm from './components/antd/form/custform/PriceForm'
import ChannelPicker from './components/antd/cascader/ChannelPicker'
import LifeCycle from './components/react/LifeCycle'
import Form1 from './components/react/form/Form1'
import Form2 from './components/react/form/Form2'
import Form3 from './components/react/form/Form3'
import Form4 from './components/react/form/Form4'
import Form5 from './components/react/form/Form5'
import Form6 from './components/react/form/Form6'

import Bar from './components/echarts/Bar'
import Bar2 from './components/echarts/Bar2'
import PunchCard from './components/echarts/PunchCard'
import LineSimple from './components/echarts/LineSimple'
import AreaBasic from './components/echarts/AreaBasic'
import LineSmooth from './components/echarts/LineSmooth'
import AreaStack from './components/echarts/AreaStack'

import WorkTime from './components/contorls/WorkTime'
import Device from './components/contorls/Device'
import Counter from './components/contorls/Counter'
import PlacesTree from './components/contorls/PlacesTree'
import PlacesTreeSelect from './components/contorls/PlacesTreeSelect'

export const menus = [{
  title: 'antd',
  children: [{
    title: 'DatePicker',
    path: '/antd/date_picker',
    component: DatePicker
  }, {
    title: 'form',
    children: [{
      title: 'normform',
      children: [{
        title: 'HorizontalLoginForm',
        path: '/antd/form/normform/horizontal_login_form',
        component: HorizontalLoginForm
      }, {
        title: 'DownloadForm',
        path: '/antd/form/normform/download_form',
        component: DownloadForm
      }]
    }, {
      title: 'custform',
      children: [{
        title: 'PriceForm',
        path: '/antd/form/custform/price_form',
        component: PriceForm
      }]
    }]
  }, {
    title: 'cascader',
    children: [{
      title: 'channel_picker',
      children: [{
        title: 'ChannelPicker',
        path: '/antd/cascader/channel_picker',
        component: ChannelPicker
      }]
    }]
  }]
}, {
  title: 'react',
  role: 'ROLE_ADMIN',
  children: [{
    title: 'lifecycle',
    path: '/react/lifecycle',
    component: LifeCycle
  }, {
    title: 'form',
    children: [{
      title: 'Form1',
      path: '/react/form/form1',
      component: Form1
    }, {
      title: 'Form2',
      path: '/react/form/form2',
      component: Form2
    }, {
      title: 'Form3',
      path: '/react/form/form3',
      component: Form3
    }, {
      title: 'Form4',
      path: '/react/form/form4',
      component: Form4
    }, {
      title: 'Form5',
      path: '/react/form/form5',
      component: Form5
    }, {
      title: 'Form6',
      role: 'ROLE_SUPER_ADMIN',
      path: '/react/form/form6',
      component: Form6
    }]
  }]
}, {
  title: 'echarts',
  children: [{
    title: 'Bar',
    path: '/echarts/bar',
    component: Bar
  }, {
    title: 'Bar2',
    role: 'ROLE_SUPER_ADMIN',
    path: '/echarts/bar2',
    component: Bar2
  }, {
    title: 'PunchCard',
    path: '/echarts/punchcard',
    component: PunchCard
  }, {
    title: 'LineSimple',
    path: '/echarts/linesimple',
    component: LineSimple
  }, {
    title: 'AreaBasic',
    path: '/echarts/areabasic',
    component: AreaBasic
  }, {
    title: 'LineSmooth',
    path: '/echarts/linesmooth',
    component: LineSmooth
  }, {
    title: 'AreaStack',
    path: '/echarts/areastack',
    component: AreaStack
  }]
}, {
  title: 'controls',
  children: [{
    title: 'WorkTime',
    path: '/contorls/worktime',
    component: WorkTime
  }, {
    title: 'Device',
    path: '/contorls/device',
    component: Device
  }, {
    title: 'Counter',
    path: '/contorls/counter',
    component: Counter
  }, {
    title: 'PlacesTree',
    path: '/contorls/placestree',
    component: PlacesTree
  }, {
    title: 'PlacesTreeSelect',
    path: '/contorls/placestreeselect',
    component: PlacesTreeSelect
  }]
}]

export const roles = ['user']

