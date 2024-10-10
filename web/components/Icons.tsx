import { Box, BoxProps, rem } from "@mantine/core";

interface IconProps extends BoxProps {
  size: number;
}

const BaseIcon = (props: IconProps & { fileName: string }) => {
  return (
    <Box
      style={{
        width: rem(props.size),
        height: rem(props.size),
        ...props.style,
      }}
      component="img"
      src={"/" + props.fileName}
    ></Box>
  );
};

export const JavaIcon = (props: IconProps) => (
  <BaseIcon fileName={"java-original.svg"} {...props} />
);
export const GoIcon = (props: IconProps) => (
  <BaseIcon fileName={"golang.svg"} {...props} />
);
