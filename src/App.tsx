import { Route, Router, Switch } from "wouter";
import { Home } from "./pages/Home";
import { Layout } from "./components/Layout";
import { Provider } from "jotai";
import { NewProject } from "./pages/NewProject";
import { ProjectDetails } from "./pages/ProjectDetails";

function App() {
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
