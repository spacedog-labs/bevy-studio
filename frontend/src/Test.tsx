import { useAuth } from "./AuthenticationContext";
import { useEffect, useState } from "react";
import styled from "styled-components";
import { User } from "./types/User";

const Test = () => {
  const authContext = useAuth();

  return (
    <Container>
      <h1>Test Suite</h1>
      <h1>{authContext.logged_in ? "logged in" : "not logged in"}</h1>
      <UserTests></UserTests>
      <FileTests></FileTests>
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
      <button onClick={get_me}>Get Me</button>
      <button onClick={update_me}>Update Me</button>
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

export default Test;
