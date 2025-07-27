import React, { useState } from "react";
import { useAtom } from "jotai";
import { useLocation } from "wouter";
import { projectsAtom } from "../store";
import { createProject } from "../utils";
import { NewProjectData } from "../types";

export const NewProject: React.FC = () => {
  const [projects, setProjects] = useAtom(projectsAtom);
  const [, setLocation] = useLocation();
  const [formData, setFormData] = useState<NewProjectData>({
    name: "",
    dataDirectory: "",
  });
  const [errors, setErrors] = useState<Partial<NewProjectData>>({});

  const validateForm = (): boolean => {
    const newErrors: Partial<NewProjectData> = {};

    if (!formData.name.trim()) {
      newErrors.name = "Project name is required";
    } else if (
      projects.some((p) => p.name.toLowerCase() === formData.name.toLowerCase())
    ) {
      newErrors.name = "A project with this name already exists";
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!validateForm()) {
      return;
    }

    const newProject = await createProject(formData, projects);
    setProjects([...projects, newProject]);
    setLocation("/");
  };

  const handleInputChange = (field: keyof NewProjectData, value: string) => {
    setFormData((prev) => ({ ...prev, [field]: value }));
    // Clear error when user starts typing
    if (errors[field]) {
      setErrors((prev) => ({ ...prev, [field]: undefined }));
    }
  };

  return (
    <div className="max-w-2xl mx-auto">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">
          Create New Project
        </h1>
        <p className="text-gray-600">
          Set up a new PocketBase project with custom configuration.
        </p>
      </div>

      <div className="bg-white shadow-sm rounded-lg border border-gray-200 p-6">
        <form onSubmit={handleSubmit} className="space-y-6">
          <div>
            <label
              htmlFor="name"
              className="block text-sm font-medium text-gray-700 mb-2"
            >
              Project Name *
            </label>
            <input
              type="text"
              id="name"
              value={formData.name}
              onChange={(e) => handleInputChange("name", e.target.value)}
              className={`block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm ${
                errors.name
                  ? "border-red-300 focus:border-red-500 focus:ring-red-500"
                  : ""
              }`}
              placeholder="e.g., E-commerce API"
            />
            {errors.name && (
              <p className="mt-1 text-sm text-red-600">{errors.name}</p>
            )}
          </div>

          <div>
            <label
              htmlFor="dataDirectory"
              className="block text-sm font-medium text-gray-700 mb-2"
            >
              Data Directory (Optional)
            </label>
            <input
              type="text"
              id="dataDirectory"
              value={formData.dataDirectory}
              onChange={(e) =>
                handleInputChange("dataDirectory", e.target.value)
              }
              className="block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
              placeholder="e.g., /data/my-project"
            />
            <p className="mt-1 text-sm text-gray-500">
              If not specified, PocketBase will use the default data directory.
            </p>
          </div>

          <div className="flex items-center justify-end space-x-4 pt-6 border-t border-gray-200">
            <button
              type="button"
              onClick={() => setLocation("/")}
              className="px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 transition-colors"
            >
              Cancel
            </button>
            <button
              type="submit"
              className="px-4 py-2 border border-transparent rounded-md text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 transition-colors"
            >
              Create Project
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};
