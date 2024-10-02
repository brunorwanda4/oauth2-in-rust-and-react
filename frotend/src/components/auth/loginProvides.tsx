import { BsGithub, BsGoogle } from "react-icons/bs"

const LoginProvides = () => {
  return (
    <div className=" flex gap-4 items-center justify-center">
      <button className=" btn btn-square rounded-none ">
        <BsGoogle size={32}/>
      </button>
      <button className=" btn btn-square rounded-none ">
        <BsGithub size={32}/>
      </button>
    </div>
  )
}

export default LoginProvides
