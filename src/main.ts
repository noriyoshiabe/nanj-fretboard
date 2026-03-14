import "./style.css";

import Fretboard from "./Fretboard";
import NotePad from "./NotePad";
import NanJ from "./NanJ";

const fretboard = new Fretboard();
const notePad = new NotePad();
const nanJ = new NanJ();

document.body.appendChild(fretboard.el);
document.body.appendChild(notePad.el);
document.body.appendChild(nanJ.el);

notePad.onClickKey = (key) => {
  console.log(key);

  switch (nanJ.state) {
  case "normal":
    nanJ.state = "angry";
    break;
  case "angry":
    nanJ.state = "happy";
    break;
  case "happy":
    nanJ.state = "normal";
    break;
  }
};

nanJ.onFadeOut = (nanJ: NanJ) => {
  console.log("Fadeout");
};
