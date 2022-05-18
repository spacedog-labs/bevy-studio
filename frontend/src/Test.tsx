import { useAuth } from "./AuthenticationContext";
import { useEffect, useState } from "react";
import styled from "styled-components";
import { Project, User } from "./types/User";
import { Button, Input } from "spacedog";
import { json } from "stream/consumers";

const Test = () => {
  const authContext = useAuth();

  return (
    <Container>
      <h1>Test Suite</h1>
      <h1>{authContext.logged_in ? "logged in" : "not logged in"}</h1>
      <UserTests></UserTests>
      <FileTests></FileTests>
      <ProjectTests></ProjectTests>
    </Container>
  );
};

const Container = styled.div`
  display: flex;
  flex-direction: column;
`;

const UserTests = () => {
  const authContext = useAuth();
  const [user, set_user] = useState<User>();

  const get_me = () => {
    fetch("/api/user/me", {
      headers: {
        Authorization: authContext.access_token,
      },
    })
      .then((response) => response.json())
      .then((user: User) => set_user(user));
  };

  const update_me = () => {
    fetch("/api/user/me", {
      headers: {
        Authorization: authContext.access_token,
      },
      body: JSON.stringify(user),
      method: "POST",
    });
  };

  return (
    <div>
      <h1>User Tests</h1>
      <input type="text" defaultValue={user?.id} disabled></input>
      <input
        type="text"
        defaultValue={user?.avatar_url}
        onChange={(e) => {
          const newUser = user;
          if (newUser != undefined) {
            newUser.avatar_url = e.target.value;
            set_user(newUser);
          }
        }}
      ></input>
      <Button
        onClick={() => {
          get_me();
        }}
      >
        test
      </Button>
      <Button onClick={update_me}>Update Me</Button>
    </div>
  );
};

const FileTests = () => {
  const authContext = useAuth();

  const [selectedFiles, setSelectedFile] = useState<FileList>();

  const upload = () => {
    const reader = new FileReader();

    reader.readAsText(selectedFiles![0]);
    reader.onload = (e) => {
      fetch(`/api/file/upload/${selectedFiles![0].name}?project_id=123`, {
        headers: {
          Authorization: authContext.access_token,
        },
        body: reader.result,
        method: "POST",
      })
        .then((response) => response.text())
        .then((text) => console.log(text));
    };
  };

  const get_file = () => {
    fetch(`/api/file/?file=${selectedFiles![0].name}&project_id=123`, {
      headers: {
        Authorization: authContext.access_token,
      },
    })
      .then((response) => response.text())
      .then((text) => console.log(text));
  };

  return (
    <div>
      <h1>Fiel Tests</h1>
      <input
        type="file"
        onChange={(e) => {
          if (!e.target.files) {
            return;
          }
          setSelectedFile(e.target.files);
        }}
      ></input>
      <button onClick={upload}>Upload</button>
      <button onClick={get_file}>Get File</button>
    </div>
  );
};

const ProjectTests = () => {
  const authContext = useAuth();
  const [project, setProject] = useState<Project>();
  const [targetProject, setTargetProject] = useState("");
  const [myProjects, setMyProjects] = useState<Project[]>();

  const get_project = () => {
    fetch(`/api/project/${targetProject}`, {
      headers: {
        Authorization: authContext.access_token,
      },
    })
      .then((response) => response.json())
      .then((project: Project) => setProject(project));
  };

  const get_projects = () => {
    fetch(`/api/project`, {
      headers: {
        Authorization: authContext.access_token,
      },
    })
      .then((response) => response.json())
      .then((projects: Project[]) => setMyProjects(projects));
  };

  const create_project = () => {
    fetch(`/api/project/create?name=${targetProject}`, {
      headers: {
        Authorization: authContext.access_token,
      },
      method: "POST",
    })
      .then((response) => response.json())
      .then((project: Project) => console.log(project));
  };

  return (
    <Container>
      <h1>Project Tests</h1>
      <Container>
        <input type="text" defaultValue={project?.name} disabled></input>
        <Input
          title="Target Project"
          value={targetProject}
          onChange={(e) => {
            setTargetProject(e.target.value);
          }}
        >
          s
        </Input>
        <Button onClick={get_project}>Get Project</Button>
        <Button onClick={create_project}>Create Project</Button>
      </Container>
      <Container>
        <Button onClick={get_projects}>Get Projects</Button>
        <h1>Projects</h1>
        <div>
          {myProjects?.map((p) => {
            const increment = () => {
              p.name += " che";
              fetch(`/api/project/update`, {
                headers: {
                  Authorization: authContext.access_token,
                },
                body: JSON.stringify(p),
                method: "POST",
              })
                .then((response) => response.json())
                .then((project: Project) => console.log(project));
            };
            return (
              <div>
                {p.name}
                <Button onClick={increment}>increment</Button>
              </div>
            );
          })}
        </div>
      </Container>
    </Container>
  );
};

export default Test;
