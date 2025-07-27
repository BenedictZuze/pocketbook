import React from "react";
import { useAtomValue } from "jotai";
import { projectsAtom } from "../store";
import { ProjectCard } from "./ProjectCard";

export const ProjectList: React.FC = () => {
  const projects = useAtomValue(projectsAtom);

  if (projects.length === 0) {
    return (
      <div className="text-center py-12">
        <div className="max-w-md mx-auto">
          <h3 className="text-lg font-medium text-gray-900 mb-2">
            No projects yet
          </h3>
          <p className="text-gray-500 mb-6">
            Create your first PocketBase project to get started.
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
      {projects.map((project) => (
        <ProjectCard key={project.id} project={project} />
      ))}
    </div>
  );
};
