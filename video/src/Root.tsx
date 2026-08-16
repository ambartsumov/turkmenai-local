import React from "react";
import { Composition } from "remotion";
import { Tour } from "./Tour";
import { FPS, W, H } from "./theme";

// ~42s vertical product tour. Language is a prop so the same timeline renders in
// RU / TK / EN without duplicating the composition.
export const RemotionRoot: React.FC = () => {
  return (
    <>
      <Composition
        id="Tour"
        component={Tour}
        durationInFrames={1080}
        fps={FPS}
        width={W}
        height={H}
        defaultProps={{ lang: "en" as const }}
      />
      <Composition
        id="TourRU"
        component={Tour}
        durationInFrames={1080}
        fps={FPS}
        width={W}
        height={H}
        defaultProps={{ lang: "ru" as const }}
      />
      <Composition
        id="TourTK"
        component={Tour}
        durationInFrames={1080}
        fps={FPS}
        width={W}
        height={H}
        defaultProps={{ lang: "tk" as const }}
      />
    </>
  );
};
