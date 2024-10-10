import { createLowlight } from "lowlight";
import ts from "highlight.js/lib/languages/typescript";
import java from "highlight.js/lib/languages/java";
import go from "highlight.js/lib/languages/go";
import json from "highlight.js/lib/languages/json";

export const lowlightFactory = () => {
  const lowlight = createLowlight();
  lowlight.register({ ts });
  lowlight.register({ java });
  lowlight.register({ json });
  lowlight.register({ go });
  return lowlight;
};
