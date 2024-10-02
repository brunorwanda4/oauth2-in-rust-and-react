import { BrowserRouter, Route, Routes } from 'react-router-dom'
import LoginForm from './components/auth/login-form'
import Register from './components/page/register'
import Profile from './components/page/profile'

function App() {

  return (
    <BrowserRouter>
      <Routes>
        <Route path='/' element={<LoginForm />}/>
        <Route path='/register' element={<Register />}/>
        <Route path='/profile' element={<Profile />}/>
      </Routes>
    </BrowserRouter>
  )
}

export default App
