import { useEffect } from "react";
import { Route, Router, Switch } from "wouter";
import { Home } from "./pages/Home";
import { Layout } from "./components/Layout";
import { Provider, useAtom } from "jotai";
import { NewProject } from "./pages/NewProject";
import { ProjectDetails } from "./pages/ProjectDetails";
import { projectsAtom } from "./store";
import { listen } from "@tauri-apps/api/event";

function App() {
  const [projects, setProjects] = useAtom(projectsAtom);

  useEffect(() => {
    const unlistenPromise = listen<{ pid: string; isHealthy: boolean }>(
      "instance-health-changed",
      (event) => {
        console.log("health changed");
        console.log(event.payload.isHealthy);
        console.log(event.payload.pid);
        setProjects((projects) =>
          projects.map((p) =>
            p.pid === event.payload.pid
              ? { ...p, isHealthy: event.payload.isHealthy }
              : p
          )
        );
        console.log(projects);
      }
    );

    // cleanup
    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, [setProjects]);

  return (
    <Provider>
      <Router>
        <Layout>
          <Switch>
            <Route path="/" component={Home} />
            <Route path="/new" component={NewProject} />
            <Route path="/project/:id" component={ProjectDetails} />
            <Route>
              <div className="text-center py-12">
                <h2 className="text-2xl font-bold text-gray-900 mb-2">
                  Page Not Found
                </h2>
                <p className="text-gray-600">
                  The page you're looking for doesn't exist.
                </p>
              </div>
            </Route>
          </Switch>
        </Layout>
      </Router>
    </Provider>
  );
}

export default App;
