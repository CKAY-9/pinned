import style from "./popup.module.scss";

const Popup = ({children}: any) => {
  return (
    <div className={style.popup}>
      <div className={style.content}>
        {children}
      </div>
    </div>
  );
}

export default Popup;
