import { useAuth } from "./AuthenticationContext";
import { Route, Routes, useSearchParams, useNavigate } from "react-router-dom";
import { useEffect, useState } from "react";

const App = () => {
  return (
    <Routes>
      <Route path="test/123" element={<Test />} />
      <Route path="auth" element={<Auth />} />
      <Route path="/" element={<Home />} />
    </Routes>
  );
};

const Home = () => {
  const authContext = useAuth();
  const [avatar_url, set_avatar_url] = useState("");

  const get_me = () => {
    fetch("/api/user/me", {
      headers: {
        Authorization: authContext.access_token,
      },
    })
      .then((response) => response.json())
      .then((json) => console.log(json));
  };
  const update_me = () => {
    fetch("/api/user/me", {
      headers: {
        Authorization: authContext.access_token,
      },
      body: JSON.stringify({ avatar_url: "wow", id: "42881380" }),
      method: "POST",
    });
  };

  const upload = () => {
    fetch("/api/projects/upload", {
      headers: {
        Authorization: authContext.access_token,
      },
      body: "peepeepoopoo",
      method: "POST",
    })
      .then((response) => response.text())
      .then((text) => console.log(text));
  };

  const get_file = () => {
    fetch("/api/projects/file?file=yolo", {
      headers: {
        Authorization: authContext.access_token,
      },
    })
      .then((response) => response.text())
      .then((text) => console.log(text));
  };

  return (
    <div>
      <h1>Home!</h1>
      <h1>{authContext.logged_in ? "logged in" : "not logged in"}</h1>
      {avatar_url != "" ? <img src={avatar_url}></img> : <></>}
      <button onClick={get_me}>Get Me</button>
      <button onClick={update_me}>Update Me</button>
      <button onClick={upload}>Upload</button>
      <button onClick={get_file}>Get File</button>
      <Login />
    </div>
  );
};

const Test = () => {
  return (
    <div>
      <h1>this is a test page!, get me back here!</h1>
      <Login />
    </div>
  );
};

const Auth = () => {
  const authContext = useAuth();
  let [searchParams] = useSearchParams();

  const code = searchParams.get("code");
  const route = searchParams.get("route");

  useEffect(() => {
    fetch(`/api/login?code=${code}`)
      .then((resp) => {
        if (resp.status <= 399) {
          return resp.text();
        }
      })
      .then((access_token) => {
        authContext.setAccessToken(access_token!);
        window.location.href = window.location.origin + route!;
      });
  }, []);

  return <div>{code}</div>;
};

const Login = () => {
  const authContext = useAuth();

  const login = async () => {
    authContext.login(authContext);
  };
  const logout = async () => {
    authContext.logout();
  };

  //if (authService.isPending()) {
  //  return <div>Loading...</div>;
  //}

  return (
    <div>
      <button onClick={login}>Login</button>
      <button onClick={logout}>Logout</button>
    </div>
  );
};

export default App;
