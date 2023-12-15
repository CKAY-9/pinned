import style from "./loading.module.scss";
import Image from "next/image";

const LoadingWheel = (props: {
  size_in_rems: number
}) => {
  return (
    <div className={style.loading}>
      <Image 
        src="/icons/loading.svg"
        alt="Loading"
        sizes="100%"
        width={0} height={0}
        style={{"width": `${props.size_in_rems}rem`, "height": `${props.size_in_rems}rem`}}
      />
    </div>
  )
}

export default LoadingWheel;
