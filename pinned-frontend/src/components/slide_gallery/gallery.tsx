"use client"

import style from "./gallery.module.scss";

const SlideGallery = ({children}: any, props: {
  gallery_title: string, 
}) => {
  return (
    <div className={style.slide_gallery}>
      <h1>{props.gallery_title}</h1>
      <div className={style.gallery}>
        {children}
      </div>
    </div>
  );
}

export default SlideGallery;
