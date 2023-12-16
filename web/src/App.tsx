import { rspc } from "./utils/rspc";
import React, { useState, useEffect } from "react";

function App() {
  const { data: d } = rspc.useQuery(["omega=what"]);
  const [htmlContent, setHtmlContent] = useState("");
  const [showHtml, setShowHtml] = useState(false);

  const fetchData = async () => {
    try {
      const response = await fetch("http://localhost:8000/api/get");
      const result = await response.text();
      setHtmlContent(result);
      setShowHtml(true);
    } catch (error) {
      console.error("Error fetching HTML content:", error);
    }
  };

  return (
    <div>
      <h1>You are running v{d}</h1>
      <button onClick={fetchData}>Fetch HTML</button>
      {showHtml && <div dangerouslySetInnerHTML={{ __html: htmlContent }} />}
    </div>
  );
}

export default App;
