import React from "react";
import { ProjectList } from "../components/ProjectList";
import { useAtom, useAtomValue } from "jotai";
import { pbAtom, projectsAtom } from "../store";
import { useSingleEffect } from "react-haiku";
import { PocketBaseProject } from "../types";

export const Home: React.FC = () => {
  const pb = useAtomValue(pbAtom);
  const [, setProjects] = useAtom(projectsAtom);
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
