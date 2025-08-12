import React, { useEffect } from "react";
import { ProjectList } from "../components/ProjectList";
import { useAtom, useAtomValue } from "jotai";
import { dbAtom, pbAtom, projectsAtom } from "../store";
import { useSingleEffect } from "react-haiku";
import { PocketBaseProject } from "../types";
import { listen } from "@tauri-apps/api/event";

export const Home: React.FC = () => {
  const pb = useAtomValue(pbAtom);
  const [, setDb] = useAtom(dbAtom);
  const [projects, setProjects] = useAtom(projectsAtom);

  useEffect(() => {
    const unlistenPromise = listen<{ pid: string; isHealthy: boolean }>(
      "instance-health-changed",
      (event) => {
        console.log("health changed");
        console.log(event.payload.isHealthy);
        console.log(event.payload.pid);
        setProjects(
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

  useSingleEffect(() => {
    const fetchProjects = async () => {
      try {
        const projects = await pb
          .collection("projects")
          .getFullList<PocketBaseProject>();
        setProjects(projects);
      } catch (error) {
        console.error("Failed to fetch projects:", error);
      }
    };
    const authUser = async () => {
      try {
        const user = await pb
          .collection("_superusers")
          .authWithPassword(
            import.meta.env.VITE_MASTER_EMAIL,
            import.meta.env.VITE_MASTER_PASSWORD
          );
        setDb(user);
      } catch (error) {
        console.error("Failed to auth user:", error);
      }
    };
    authUser();
    fetchProjects();
  });
  return (
    <div>
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">
          PocketBase Projects
        </h1>
        <p className="text-gray-600">
          Manage your PocketBase backend projects from one central dashboard.
        </p>
      </div>

      <ProjectList />
    </div>
  );
};
