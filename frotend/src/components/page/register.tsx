import { useState } from 'react'
import { BiSolidError } from 'react-icons/bi';
import { BsCheckCircle } from 'react-icons/bs';
import { NavLink } from 'react-router-dom';
import LoginProvides from '../auth/loginProvides';

const Register = () => {
    const [email , setEmail] = useState("");
    const [password , setPassword] = useState("");
    
    const [error , setError ] = useState("");
    const [success , setSuccess] = useState("");

    
  return (
    <div className=' min-h-screen w-full items-center flex justify-center flex-col gap-4'>
        <h2 className=' font-bold text-2xl uppercase'>Create account</h2>
      <form className=' w-96 bg-base-300 flex flex-col gap-4 p-6 shadow-md'>
            <div className=' flex flex-col gap-2'>
                <label htmlFor="">Email</label>
                <input className=' input input-accent rounded-none' onChange={(e) => setEmail(e.target.value)} placeholder='Enter your Email' type="text" name="" id="email" />
            </div>
            <div className=' flex flex-col gap-2'>
                <label htmlFor="password">Password</label>
                <input className=' input input-accent rounded-none' onChange={(e) => setPassword(e.target.value)} placeholder='' type="password" name="" id="password" />
            </div>
            <div className=' flex flex-col gap-2'>
                <label htmlFor="password">Password Again</label>
                <input className=' input input-accent rounded-none' onChange={(e) => setPassword(e.target.value)} placeholder='' type="password" name="" id="password" />
            </div>
        {error && 
            <div className='text-error font-semibold flex gap-2'>
                <BiSolidError />
               <p>{error}</p>
            </div>
        }
        {success && (
            <div className=' flex gap-2 text-success'>
                <BsCheckCircle/>
                <p>{success}</p>
            </div>
        )}
        <button className=' btn rounded-none btn-accent'>Login</button>
        <div className=' link link-accent'>
            <NavLink to={"/"}>
              Login
            </NavLink>
        </div>
      <div>
        <LoginProvides />
      </div>
      </form>
    </div>
  )
}

export default Register

