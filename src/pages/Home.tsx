import React from "react";
import { ProjectList } from "../components/ProjectList";

export const Home: React.FC = () => {
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
