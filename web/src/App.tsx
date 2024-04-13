import { Button } from "./components/ui/button";
import { rspc } from "./utils/rspc";
import React, { useState, useEffect } from "react";
import { CreateTodoArgs, Task, TaskStatus } from "./utils/api";

function App() {
  const queryClient = rspc.useContext().queryClient;

  const { data: d } = rspc.useQuery(["todo.get_all"]);
  const { mutateAsync: createTodo } = rspc.useMutation("todo.create", {
    onSuccess: () => {
      queryClient.invalidateQueries(["todo.get_all"]);
    },
  });

  const { mutateAsync: updateTodo } = rspc.useMutation("todo.update", {
    onSuccess: () => {
      queryClient.invalidateQueries(["todo.get_all"]);
    },
  });

  const [title, settitle] = useState("");
  const [status, setstatus] = useState<TaskStatus>("Undone");
  const [update, setupdate] = useState(false);

  const onSubmit = async () => {
    console.log(title, status);
    await createTodo({
      title,
      status,
    });
    settitle("");
  };

  return (
    <div>
      <div className="bg-red-200 p-3 rounded-xl flex flex-col mt-4">
        <input
          id="title"
          type="text"
          placeholder="Type here"
          className="input input-bordered w-full mb-3 p-3 rounded-xl"
          onChange={(e) => settitle(e.target.value)}
        />
        <select
          id="status"
          className="select select-bordered w-full mb-3 p-3 rounded-xl"
          defaultValue={"Undone"}
          onChange={(e) => setstatus(e.target.value as TaskStatus)}
        >
          <option disabled>Status</option>
          <option>Done</option>
          <option>Undone</option>
        </select>
        <button
          className="bg-red-500 p-3 rounded-xl text-white"
          onClick={onSubmit}
        >
          Submit
        </button>
      </div>
      <div className="flex flex-col m-2 p-5 rounded-2xl bg-red-500">
        {d?.map((s) => {
          return <ListItem key={s.id} s={s} />;
        })}
      </div>
    </div>
  );
}

export default App;

const ListItem = ({ s }: { s: Task }) => {
  const queryClient = rspc.useContext().queryClient;

  const [title, settitle] = useState(s.title);
  const [status, setstatus] = useState<TaskStatus>(s.status);
  const [update, setupdate] = useState(false);

  const { mutateAsync: updateTodo } = rspc.useMutation("todo.update", {
    onSuccess: () => {
      queryClient.invalidateQueries(["todo.get_all"]);
    },
  });

  const onSubmit = async () => {
    console.log(title, status);
    await updateTodo({
      id: s.id,
      status,
      title,
    });
    settitle("");
    setupdate(false);
  };
  return (
    <div className="mb-3">
      <div
        className="w-5 h-5 rounded-full bg-blue-600 cursor-pointer text-white flex items-center justify-center text-sm mb-3"
        onClick={() => setupdate(!update)}
      >
        U
      </div>
      {!update ? (
        <div className="mb-3">
          <div
            id={s.id.toString()}
            className="w-full bg-purple-500 text-white p-3 rounded-xl flex flex-row justify-between items-center"
          >
            <p>{s.title}</p>
            <p className="bg-yellow-600 text-white font-bold w-fit p-3 rounded-xl">
              {s.status}
            </p>
          </div>
        </div>
      ) : (
        <div className="">
          <input
            id="title"
            type="text"
            placeholder="Type here"
            className="input input-bordered w-full mb-3 p-3 rounded-xl"
            defaultValue={s.title}
            onChange={(e) => settitle(e.target.value)}
          />
          <select
            id="status"
            className="select select-bordered w-full mb-3 p-3 rounded-xl"
            defaultValue={s.status}
            onChange={(e) => setstatus(e.target.value as TaskStatus)}
          >
            <option disabled>Status</option>
            <option>Done</option>
            <option>Undone</option>
          </select>
          <button
            className="bg-red-900 p-3 rounded-xl text-white"
            onClick={onSubmit}
          >
            Submit
          </button>
        </div>
      )}
    </div>
  );
};
