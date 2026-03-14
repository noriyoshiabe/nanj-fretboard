import "./style.css";

import Fretboard from "./Fretboard";
import NotePad from "./NotePad";

const fretboard = new Fretboard();
const notePad = new NotePad();

document.body.appendChild(fretboard.el);
document.body.appendChild(notePad.el);

notePad.onClickKey = (key) => {
  console.log(key);
};
