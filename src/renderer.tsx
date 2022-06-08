import React from "react";
import ReactDOM from "react-dom";

class RootDiv extends React.Component<{}, {}> {
  constructor(props: {}) {
    super(props);
  }
  render = () => {
    return <>Hello World</>;
  };
}

ReactDOM.render(<RootDiv />, document.getElementById("root"));
